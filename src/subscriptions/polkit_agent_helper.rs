use cosmic::iced::{self, Subscription};
use futures::SinkExt;
use std::ops::ControlFlow;
use std::path::Path;
use std::process::Stdio;
use std::{fmt, io};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::UnixStream;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::process::Command;
use tokio::select;
use tokio::sync::mpsc::{self, Sender};

const HELPER_BIN_PATH: Option<&str> = option_env!("POLKIT_AGENT_HELPER_1");
const HELPER_SOCKET_PATH: &str = "/run/polkit/agent-helper.socket";

#[derive(Clone, Debug)]
pub enum Event {
    Failed,
    Responder(Responder),
    Request(String, bool),
    ShowError(String),
    ShowDebug(String),
    Complete(bool),
}

pub fn subscription(pw_name: &str, cookie: &str) -> Subscription<Event> {
    let args = (pw_name.to_owned(), cookie.to_owned());
    Subscription::run_with(args, |args| {
        let pw_name = args.0.to_owned();
        let cookie = args.1.to_owned();

        iced::stream::channel(16, async move |mut output| {
            for _ in 0..3 {
                let ControlFlow::Break(successful) =
                    try_authenticate(&pw_name, &cookie, &mut output).await
                else {
                    continue;
                };

                if successful {
                    log::debug!("authenticated successfully");
                    return;
                };

                log::debug!("retrying authentication");
            }

            log::info!("retries exhausted");

            let _ = output.send(Event::Failed).await;
        })
    })
}

#[derive(Clone)]
pub struct Responder {
    sender: Sender<String>,
}

impl Responder {
    pub async fn response(&self, resp: &str) -> Result<(), ()> {
        self.sender.send(resp.to_owned()).await.map_err(|_| ())?;

        Ok(())
    }
}

impl fmt::Debug for Responder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Responder")
    }
}

async fn try_authenticate(
    pw_name: &str,
    cookie: &str,
    output: &mut futures::channel::mpsc::Sender<Event>,
) -> ControlFlow<bool> {
    let mut agent_helper = match AgentHelper::new(pw_name, cookie).await {
        Ok(agent_helper) => agent_helper,
        Err(err) => {
            log::error!("failed to create helper, {}", err.kind());
            let _ = output.send(Event::Failed).await;

            return ControlFlow::Break(false);
        }
    };

    let (sender, mut receiver) = mpsc::channel::<String>(16);
    let _ = output.send(Event::Responder(Responder { sender })).await;

    loop {
        select! {
            next = agent_helper.next() => agent_next(next, output).await?,
            Some(msg) = receiver.recv() =>
                responder_next(
                    &msg,
                    &mut agent_helper,
                    output
                )
                    .await
                    .map_break(|_| false)?,
        }
    }
}

async fn agent_next(
    next: io::Result<Option<Event>>,
    output: &mut futures::channel::mpsc::Sender<Event>,
) -> ControlFlow<bool> {
    match next {
        Ok(Some(event)) => {
            let Event::Complete(successful) = event else {
                let _ = output.send(event).await;
                return ControlFlow::Continue(());
            };

            log::debug!("got completed event (successful: {successful}), exiting");
            let _ = output.send(event).await;

            ControlFlow::Break(successful)
        }
        Ok(None) => {
            log::debug!("no next message from helper, exiting");
            ControlFlow::Break(false)
        }
        Err(err) => {
            log::error!("failed to get next message from helper: {}", err.kind());
            let _ = output.send(Event::Failed).await;
            ControlFlow::Break(false)
        }
    }
}

async fn responder_next(
    msg: &str,
    agent_helper: &mut AgentHelper,
    output: &mut futures::channel::mpsc::Sender<Event>,
) -> ControlFlow<()> {
    if let Err(err) = agent_helper.write(msg).await {
        log::error!(
            "failed to send message from the responder to the auth helper, error: {}",
            err.kind()
        );
        let _ = output.send(Event::Failed).await;

        ControlFlow::Break(())
    } else {
        ControlFlow::Continue(())
    }
}

enum AgentHelper {
    Bin {
        _child: Box<tokio::process::Child>,
        stdout: BufReader<tokio::process::ChildStdout>,
        stdin: BufWriter<tokio::process::ChildStdin>,
    },
    Socket {
        read_half: BufReader<OwnedReadHalf>,
        write_half: OwnedWriteHalf,
    },
}

impl AgentHelper {
    async fn new(pw_name: &str, cookie: &str) -> io::Result<Self> {
        let mut agent_helper = if Path::new(HELPER_SOCKET_PATH).exists() {
            Self::new_socket(pw_name).await?
        } else {
            Self::new_bin(pw_name).await?
        };

        agent_helper.write(cookie).await?;

        Ok(agent_helper)
    }

    async fn new_socket(pw_name: &str) -> io::Result<Self> {
        log::info!("using socket");

        let stream = UnixStream::connect(HELPER_SOCKET_PATH).await?;
        let (read, write_half) = stream.into_split();

        let read_half = BufReader::new(read);

        let mut agent_helper = Self::Socket {
            read_half,
            write_half,
        };

        agent_helper.write(pw_name).await?;

        Ok(agent_helper)
    }

    async fn new_bin(pw_name: &str) -> io::Result<Self> {
        log::info!("using binary");

        let helper_bin_path = HELPER_BIN_PATH.unwrap_or("/usr/libexec/polkit-agent-helper-1");
        log::trace!("using helper binary from: {helper_bin_path}");

        let mut child = Command::new(helper_bin_path)
            .kill_on_drop(true)
            .arg(pw_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = BufWriter::new(child.stdin.take().unwrap());
        let stdout = BufReader::new(child.stdout.take().unwrap());

        Ok(Self::Bin {
            _child: Box::new(child),
            stdin,
            stdout,
        })
    }

    async fn next(&mut self) -> io::Result<Option<Event>> {
        let reader: &mut (dyn Unpin + Send + Sync + AsyncBufRead) = match self {
            Self::Bin { stdout, .. } => stdout,
            Self::Socket { read_half, .. } => read_half,
        };

        let mut line = String::new();
        while reader.read_line(&mut line).await? != 0 {
            match event(&line) {
                Ok(event) => return Ok(Some(event)),
                Err(prefix) => {
                    log::error!(
                        "Unknown prefix: '{prefix}' in line '{line}' from 'polkit-agent-helper-1'"
                    );
                    continue;
                }
            }
        }

        Ok(None)
    }

    async fn write(&mut self, msg: &str) -> io::Result<()> {
        let msg = format!("{msg}\n");

        let writer: &mut (dyn Unpin + Send + Sync + AsyncWrite) = match self {
            Self::Bin { stdin, .. } => stdin,
            Self::Socket { write_half, .. } => write_half,
        };

        writer.write_all(msg.as_bytes()).await?;
        writer.flush().await?;

        Ok(())
    }
}

fn event(line: &str) -> Result<Event, &str> {
    let line = line.trim();
    let (prefix, rest) = line.split_once(' ').unwrap_or((line, ""));

    Ok(match prefix {
        "PAM_PROMPT_ECHO_OFF" => Event::Request(rest.to_string(), false),
        "PAM_PROMPT_ECHO_ON" => Event::Request(rest.to_string(), true),
        "PAM_ERROR_MSG" => Event::ShowError(rest.to_string()),
        "PAM_TEXT_INFO" => Event::ShowDebug(rest.to_string()),
        "SUCCESS" => Event::Complete(true),
        "FAILURE" => Event::Complete(false),
        unknown_prefix => {
            return Err(unknown_prefix);
        }
    })
}
