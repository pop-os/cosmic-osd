use cosmic::iced::Subscription;
use futures::lock::Mutex;
use futures::stream;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use std::{fmt, io};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{UnixStream, unix};
use tokio::process::{ChildStdin, Command};

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

pub fn subscription(pw_name: &str, cookie: &str, retry: u32) -> Subscription<Event> {
    // TODO: Avoid clone?
    let args = Some((pw_name.to_owned(), cookie.to_owned()));
    let name = format!("agent-helper-{}-{}", cookie, retry);
    Subscription::run_with((name, args), |(_, args)| {
        let mut args = args.clone();
        stream::unfold(None::<AgentHelper>, move |agent_helper| {
            let args = args.take();
            async move {
                let mut agent_helper = match agent_helper {
                    Some(h) => h,
                    None => {
                        let (pw_name, cookie) = args.unwrap();
                        match AgentHelper::new(&pw_name, &cookie).await {
                            Ok((helper, responder)) => {
                                return Some((Event::Responder(responder), Some(helper)));
                            }
                            Err(err) => {
                                log::error!("creating polkit agent helper: {}", err);
                                return Some((Event::Failed, None));
                            }
                        }
                    }
                };

                match agent_helper.connection.next().await {
                    Ok(Some(msg)) => Some((msg, Some(agent_helper))),
                    Ok(None) => None,
                    Err(err) => {
                        log::error!("reading from polkit agent helper: {}", err);
                        Some((Event::Failed, Some(agent_helper)))
                    }
                }
            }
        })
    })
}

#[derive(Clone)]
pub struct Responder {
    connection: Arc<Mutex<Box<dyn AsyncWrite + Unpin + Send>>>,
}

impl Responder {
    fn new_bin(stdin: BufWriter<ChildStdin>) -> Self {
        Self {
            connection: Arc::new(Mutex::new(Box::new(stdin))),
        }
    }

    fn new_socket(writer: unix::OwnedWriteHalf) -> Self {
        Self {
            connection: Arc::new(Mutex::new(Box::new(writer))),
        }
    }

    pub async fn response(&self, resp: &str) -> Result<(), ()> {
        let resp = format!("{resp}\n").to_owned();
        let resp = resp.as_bytes();

        let mut connection = self.connection.lock().await;

        connection.write_all(resp).await.map_err(|_| ())?;
        connection.flush().await.map_err(|_| ())?;

        Ok(())
    }
}

impl fmt::Debug for Responder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Responder")
    }
}

struct AgentHelper {
    connection: Connection,
}

impl AgentHelper {
    async fn new(pw_name: &str, cookie: &str) -> io::Result<(Self, Responder)> {
        let (connection, responder) = if Path::new(HELPER_SOCKET_PATH).exists() {
            log::info!("using socket");

            let (socket, writer) = ConnectionSocket::new().await?;
            let responder = Responder::new_socket(writer);

            responder.response(pw_name).await.unwrap();

            (Connection::Socket(socket), responder)
        } else {
            log::info!("using binary");

            let (binary, stdin) = ConnectionBin::new(pw_name).await?;
            (Connection::Bin(binary), Responder::new_bin(stdin))
        };

        responder.response(cookie).await.unwrap();

        Ok((Self { connection }, responder))
    }
}

enum Connection {
    Bin(ConnectionBin),
    Socket(ConnectionSocket),
}

impl Connection {
    async fn next(&mut self) -> io::Result<Option<Event>> {
        let reader: &mut (dyn Unpin + Send + Sync + AsyncBufRead) = match self {
            Connection::Bin(bin_helper) => &mut bin_helper.stdout,
            Connection::Socket(socket_helper) => &mut socket_helper.0,
        };

        let mut line = String::new();
        while reader.read_line(&mut line).await? != 0 {
            match Self::event(&line) {
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
}

struct ConnectionBin {
    _child: tokio::process::Child,
    stdout: BufReader<tokio::process::ChildStdout>,
}

impl ConnectionBin {
    async fn new(pw_name: &str) -> io::Result<(Self, BufWriter<ChildStdin>)> {
        let helper_bin_path = HELPER_BIN_PATH.unwrap_or("/usr/libexec/polkit-agent-helper-1");
        log::trace!("using helper binary from: {helper_bin_path}");

        let mut child = Command::new(helper_bin_path)
            .arg(pw_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = BufWriter::new(child.stdin.take().unwrap());
        let stdout = BufReader::new(child.stdout.take().unwrap());

        Ok((
            Self {
                _child: child,
                stdout,
            },
            stdin,
        ))
    }
}

struct ConnectionSocket(BufReader<unix::OwnedReadHalf>);

impl ConnectionSocket {
    async fn new() -> io::Result<(Self, unix::OwnedWriteHalf)> {
        let socket = UnixStream::connect(HELPER_SOCKET_PATH).await?;

        let (reader, writer) = socket.into_split();

        Ok((Self(BufReader::new(reader)), writer))
    }
}
