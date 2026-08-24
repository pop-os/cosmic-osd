use pam_client::{Context, ConversationHandler, ErrorCode, Flag};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::io::{self, BufRead, Read, Write};
use std::mem::MaybeUninit;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};

#[derive(Debug)]
enum Outcome {
    Success,
    Failure,
}

enum Event {
    Prompt {
        text: String,
        echo: bool,
        reply: mpsc::Sender<String>,
    },

    Info(String),
    Error(String),
    Response(String),
    PeerGone,

    Finished {
        service: &'static str,
        authoritative: bool,
        authenticated: bool,
    },
}

struct Stack {
    service: &'static str,
    interactive: bool,
    authoritative: bool,
}

struct Conversation {
    stack: &'static Stack,
    events: mpsc::Sender<Event>,
    cancelled: Arc<AtomicBool>,
    claimed: Option<mpsc::Sender<()>>,
}

const FINGERPRINT_CLAIM_TIMEOUT: Duration = Duration::from_millis(1000);
const RETRY_DELAY: Duration = Duration::from_secs(1);
const MAX_LINE_LEN: u64 = 4096;
const PAM_CONFIG_DIRS: [&str; 2] = ["/etc/pam.d", "/usr/lib/pam.d"];

const PASSWORD_STACK: Stack = Stack {
    service: "polkit-1",
    interactive: true,
    authoritative: true,
};

const FINGERPRINT_STACK: Stack = Stack {
    service: "cosmic-osd-fingerprint",
    interactive: false,
    authoritative: false,
};

impl Conversation {
    fn prompt(&self, prompt: &CStr, echo: bool) -> Result<CString, ErrorCode> {
        if let Some(claimed) = &self.claimed {
            let _ = claimed.send(());
        }

        if !self.stack.interactive {
            log::warn!(
                "{}: refusing a prompt from a stack that is not allowed to prompt",
                self.stack.service
            );

            return Err(ErrorCode::CONV_ERR);
        }

        if self.cancelled.load(Ordering::Relaxed) {
            return Err(ErrorCode::CONV_ERR);
        }

        let (reply, replies) = mpsc::channel();

        self.events
            .send(Event::Prompt {
                text: prompt.to_string_lossy().into_owned(),
                echo,
                reply,
            })
            .map_err(|_| ErrorCode::CONV_ERR)?;

        match replies.recv() {
            Ok(response) => CString::new(response).map_err(|_| ErrorCode::CONV_ERR),
            Err(mpsc::RecvError) => Err(ErrorCode::CONV_ERR),
        }
    }

    fn notify(&self, event: impl FnOnce(String) -> Event, message: &CStr) {
        if let Some(claimed) = &self.claimed {
            let _ = claimed.send(());
        }

        if self.cancelled.load(Ordering::Relaxed) {
            return;
        }

        let _ = self
            .events
            .send(event(message.to_string_lossy().into_owned()));
    }
}

impl ConversationHandler for Conversation {
    fn prompt_echo_on(&mut self, prompt: &CStr) -> Result<CString, ErrorCode> {
        self.prompt(prompt, true)
    }

    fn prompt_echo_off(&mut self, prompt: &CStr) -> Result<CString, ErrorCode> {
        self.prompt(prompt, false)
    }

    fn text_info(&mut self, message: &CStr) {
        self.notify(Event::Info, message);
    }

    fn error_msg(&mut self, message: &CStr) {
        self.notify(Event::Error, message);
    }
}

fn run_stack(
    stack: &'static Stack,
    username: &str,
    events: mpsc::Sender<Event>,
    cancelled: Arc<AtomicBool>,
    claimed: Option<mpsc::Sender<()>>,
) -> bool {
    let conversation = Conversation {
        stack,
        events,
        cancelled,
        claimed,
    };

    let mut context = match Context::new(stack.service, Some(username), conversation) {
        Ok(context) => context,

        Err(err) => {
            log::warn!("{}: pam_start failed: {err}", stack.service);

            return false;
        }
    };

    if let Err(err) = context.set_ruser(Some(username)) {
        log::warn!("{}: could not set PAM_RUSER: {err}", stack.service);

        return false;
    }

    if let Err(err) = context.authenticate(Flag::NONE) {
        log::info!("{}: authentication failed: {err}", stack.service);

        return false;
    }

    if let Err(err) = context.acct_mgmt(Flag::NONE) {
        log::info!("{}: account management failed: {err}", stack.service);

        return false;
    }

    let authenticated = match context.user() {
        Ok(authenticated) => authenticated,

        Err(err) => {
            log::warn!("{}: could not read PAM_USER: {err}", stack.service);

            return false;
        }
    };

    if authenticated != username {
        log::warn!(
            "{}: asked to authenticate {username:?}, but PAM authenticated {authenticated:?}",
            stack.service
        );

        return false;
    }

    true
}

async fn fingerprint_available(service: &str, username: &str) -> zbus::Result<bool> {
    let configured = PAM_CONFIG_DIRS
        .iter()
        .any(|dir| Path::new(dir).join(service).exists());

    if !configured {
        return Ok(false);
    }

    let connection = zbus::Connection::system().await?;

    let manager = zbus::Proxy::new(
        &connection,
        "net.reactivated.Fprint",
        "/net/reactivated/Fprint/Manager",
        "net.reactivated.Fprint.Manager",
    )
    .await?;

    let device_path: OwnedObjectPath = manager.call("GetDefaultDevice", &()).await?;

    let device = zbus::Proxy::new(
        &connection,
        "net.reactivated.Fprint",
        device_path,
        "net.reactivated.Fprint.Device",
    )
    .await?;

    let fingers: Vec<String> = device.call("ListEnrolledFingers", &(username,)).await?;

    Ok(!fingers.is_empty())
}

// Get the UID of the process at the other end of the connection to pass to polkitd (which
// requires the UID of the agent that registered, so this has to come from the kernel).
// The socket is using Accept=yes, so systemd hands us the accepted connection as stdin/out
fn peer_uid() -> io::Result<u32> {
    let mut cred = MaybeUninit::<libc::ucred>::uninit();

    let cred_len = libc::socklen_t::try_from(size_of::<libc::ucred>())
        .expect("struct ucred fits in a socklen_t");

    let mut len = cred_len;

    let rc = unsafe {
        libc::getsockopt(
            libc::STDIN_FILENO,
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            cred.as_mut_ptr().cast(),
            &raw mut len,
        )
    };

    if rc == -1 {
        return Err(io::Error::last_os_error());
    }

    if len != cred_len {
        return Err(io::Error::other("SO_PEERCRED returned an unexpected size"));
    }

    Ok(unsafe { cred.assume_init() }.uid)
}

// Read a line while ensuring \n appears before the cap (MAX_LINE_LEN) to prevent memory
// peaks held at one instant.
fn read_capped_line<R: BufRead>(reader: &mut R, what: &str) -> io::Result<String> {
    let mut line = String::new();

    if reader.by_ref().take(MAX_LINE_LEN).read_line(&mut line)? == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!("peer closed the connection before sending the {what}"),
        ));
    }

    if !line.ends_with('\n') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{what} is longer than {MAX_LINE_LEN} bytes"),
        ));
    }

    Ok(line.trim_end().to_owned())
}

fn read_request() -> io::Result<(String, String)> {
    let mut stdin = io::stdin().lock();
    let username = read_capped_line(&mut stdin, "username")?;
    let cookie = read_capped_line(&mut stdin, "cookie")?;

    Ok((username, cookie))
}

fn send_line(line: &str) -> io::Result<()> {
    let mut stdout = io::stdout().lock();

    writeln!(stdout, "{line}")?;
    stdout.flush()
}

fn send_message(prefix: &str, text: &str) -> io::Result<()> {
    let text: String = text
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();

    send_line(&format!("{prefix} {}", text.trim_end()))
}

// Hand successful auth to polkitd
async fn complete_polkit_authentication(
    cookie: &str,
    auth_uid: u32,
    agent_uid: u32,
) -> zbus::Result<()> {
    let connection = zbus::Connection::system().await?;

    let proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.PolicyKit1",
        "/org/freedesktop/PolicyKit1/Authority",
        "org.freedesktop.PolicyKit1.Authority",
    )
    .await?;

    let mut details: HashMap<String, OwnedValue> = HashMap::new();
    details.insert("uid".into(), OwnedValue::from(auth_uid));

    let identity = ("unix-user", details);

    proxy
        .call_method(
            "AuthenticationAgentResponse2",
            &(agent_uid, cookie, identity),
        )
        .await?;

    Ok(())
}

fn spawn_stacks(
    username: &str,
    fingerprint: bool,
    events: &mpsc::Sender<Event>,
    cancelled: &Arc<AtomicBool>,
) {
    let mut stacks = Vec::with_capacity(2);
    let (claimed_tx, claimed) = mpsc::channel();

    if fingerprint {
        stacks.push((&FINGERPRINT_STACK, Some(claimed_tx)));
    }

    stacks.push((&PASSWORD_STACK, None));

    for (stack, claimed_tx) in stacks {
        if stack.authoritative
            && fingerprint
            && claimed.recv_timeout(FINGERPRINT_CLAIM_TIMEOUT).is_err()
        {
            log::debug!("starting the password stack without waiting for the reader");
        }

        let events = events.clone();
        let cancelled = cancelled.clone();
        let username = username.to_owned();

        std::thread::spawn(move || {
            let mut claimed_tx = claimed_tx;

            loop {
                let authenticated = run_stack(
                    stack,
                    &username,
                    events.clone(),
                    cancelled.clone(),
                    claimed_tx.take(),
                );

                if cancelled.load(Ordering::Relaxed) {
                    return;
                }

                if authenticated || stack.authoritative {
                    let _ = events.send(Event::Finished {
                        service: stack.service,
                        authoritative: stack.authoritative,
                        authenticated,
                    });

                    return;
                }

                log::debug!("{}: re-arming", stack.service);
                std::thread::sleep(RETRY_DELAY);
            }
        });
    }
}

fn spawn_input_reader(events: mpsc::Sender<Event>) {
    std::thread::spawn(move || {
        let mut stdin = io::stdin().lock();

        loop {
            match read_capped_line(&mut stdin, "response") {
                Ok(line) => {
                    if events.send(Event::Response(line)).is_err() {
                        break;
                    }
                }
                Err(err) => {
                    log::debug!("no more input from the agent: {}", err.kind());
                    let _ = events.send(Event::PeerGone);
                    break;
                }
            }
        }
    });
}

fn run_conversation(incoming: &mpsc::Receiver<Event>, auth_uid: u32) -> io::Result<Outcome> {
    let mut pending_reply: Option<mpsc::Sender<String>> = None;

    loop {
        let Ok(event) = incoming.recv() else {
            return Ok(Outcome::Failure);
        };

        match event {
            Event::Prompt { text, echo, reply } => {
                let prefix = if echo {
                    "PAM_PROMPT_ECHO_ON"
                } else {
                    "PAM_PROMPT_ECHO_OFF"
                };

                send_message(prefix, &text)?;
                pending_reply = Some(reply);
            }
            Event::Info(message) => send_message("PAM_TEXT_INFO", &message)?,
            Event::Error(message) => send_message("PAM_ERROR_MSG", &message)?,
            Event::Response(response) => match pending_reply.take() {
                Some(reply) => {
                    let _ = reply.send(response);
                }
                None => log::debug!("discarding input, no prompt is outstanding"),
            },
            Event::PeerGone => {
                log::info!("the agent closed the connection");
                return Ok(Outcome::Failure);
            }
            Event::Finished {
                service,
                authoritative,
                authenticated,
            } => {
                if authenticated {
                    log::info!("{service}: authenticated uid {auth_uid}");
                    return Ok(Outcome::Success);
                }

                if authoritative {
                    return Ok(Outcome::Failure);
                }

                log::debug!("{service}: did not authenticate, continuing");
            }
        }
    }
}

async fn authenticate() -> io::Result<Outcome> {
    let agent_uid = peer_uid()?;
    let (username, cookie) = read_request()?;

    let Some(user) = uzers::get_user_by_name(&username) else {
        log::warn!("agent (uid {agent_uid}) asked to authenticate an unknown user");

        return Ok(Outcome::Failure);
    };

    let auth_uid = user.uid();

    log::info!("agent (uid {agent_uid}) is authenticating uid {auth_uid}");

    let (events, incoming) = mpsc::channel();
    let cancelled = Arc::new(AtomicBool::new(false));

    let fingerprint = fingerprint_available(FINGERPRINT_STACK.service, &username)
        .await
        .unwrap_or_else(|err| {
            log::info!("fprintd is unavailable: {err}");

            false
        });

    if !fingerprint {
        log::debug!("not running the {} stack", FINGERPRINT_STACK.service);
    }

    spawn_stacks(&username, fingerprint, &events, &cancelled);
    spawn_input_reader(events.clone());
    drop(events);

    let outcome = run_conversation(&incoming, auth_uid);

    cancelled.store(true, Ordering::Relaxed);

    if matches!(outcome, Ok(Outcome::Success)) {
        complete_polkit_authentication(&cookie, auth_uid, agent_uid)
            .await
            .map_err(|err| io::Error::other(format!("polkitd rejected the response: {err}")))?;
    }

    outcome
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let outcome = match authenticate().await {
        Ok(outcome) => outcome,

        Err(err)
            if matches!(
                err.kind(),
                io::ErrorKind::BrokenPipe | io::ErrorKind::UnexpectedEof
            ) =>
        {
            log::info!("the agent went away: {}", err.kind());

            Outcome::Failure
        }

        Err(err) => {
            log::error!("authentication attempt aborted: {err}");

            Outcome::Failure
        }
    };

    if let Err(err) = send_line(&format!("{outcome:?}").to_uppercase()) {
        log::warn!("could not report the result to the agent: {}", err.kind());
    }
}
