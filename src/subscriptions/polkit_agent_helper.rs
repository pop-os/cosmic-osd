use cosmic::iced::{self, futures::future};
use std::{fmt, io, process::Stdio, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
    sync::Mutex,
};

const HELPER_PATH: Option<&str> = option_env!("POLKIT_AGENT_HELPER_1");

#[derive(Clone, Debug)]
pub enum Event {
    Failed,
    Responder(Responder),
    Request(String, bool),
    ShowError(String),
    ShowDebug(String),
    Complete(bool),
}

pub fn subscription(pw_name: &str, cookie: &str, retry: u32) -> iced::Subscription<Event> {
    // TODO: Avoid clone?
    let mut args = Some((pw_name.to_owned(), cookie.to_owned()));
    let name = format!("agent-helper-{}-{}", cookie, retry);
    iced::subscription::unfold(name, None::<AgentHelper>, move |agent_helper| {
        let args = args.take();
        async move {
            if let Some(mut agent_helper) = agent_helper {
                let msg = agent_helper.next().await.unwrap_or_else(|err| {
                    log::error!("reading from polkit agent helper: {}", err);
                    Some(Event::Failed)
                });
                if let Some(msg) = msg {
                    (msg, Some(agent_helper))
                } else {
                    future::pending().await
                }
            } else {
                let (pw_name, cookie) = args.unwrap();
                match AgentHelper::new(&pw_name, &cookie).await {
                    Ok((helper, responder)) => (Event::Responder(responder), Some(helper)),
                    Err(err) => {
                        log::error!("creating polkit agent helper: {}", err);
                        (Event::Failed, None)
                    }
                }
            }
        }
    })
}

struct AgentHelper {
    _child: tokio::process::Child,
    stdout: BufReader<tokio::process::ChildStdout>,
}

impl AgentHelper {
    async fn new(pw_name: &str, cookie: &str) -> io::Result<(Self, Responder)> {
        let mut child = Command::new(HELPER_PATH.unwrap_or("/usr/libexec/polkit-agent-helper-1"))
            .arg(pw_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let responder = Responder {
            stdin: Arc::new(Mutex::new(child.stdin.take().unwrap())),
        };
        responder.response(cookie).await?;
        Ok((
            Self {
                stdout: BufReader::new(child.stdout.take().unwrap()),
                _child: child,
            },
            responder,
        ))
    }

    async fn next(&mut self) -> io::Result<Option<Event>> {
        let mut line = String::new();
        while self.stdout.read_line(&mut line).await? != 0 {
            let line = line.trim();
            let (prefix, rest) = line.split_once(' ').unwrap_or((line, ""));
            return Ok(Some(match prefix {
                "PAM_PROMPT_ECHO_OFF" => Event::Request(rest.to_string(), false),
                "PAM_PROMPT_ECHO_ON" => Event::Request(rest.to_string(), true),
                "PAM_ERROR_MSG" => Event::ShowError(rest.to_string()),
                "PAM_TEXT_INFO" => Event::ShowDebug(rest.to_string()),
                "SUCCESS" => Event::Complete(true),
                "FAILURE" => Event::Complete(false),
                _ => {
                    log::error!("Unknown line '{}' from 'polkit-agent-helper-1'", line);
                    continue;
                }
            }));
        }
        Ok(None)
    }
}

#[derive(Clone)]
pub struct Responder {
    stdin: Arc<Mutex<tokio::process::ChildStdin>>,
}

impl Responder {
    pub async fn response(&self, resp: &str) -> io::Result<()> {
        let mut stdin = self.stdin.lock().await;
        stdin.write_all(resp.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        Ok(())
    }
}

impl fmt::Debug for Responder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Responder")
    }
}
