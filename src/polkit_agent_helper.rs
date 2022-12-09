// TODO more elmy design?
// - return immediately
// - how to split subscription and sender?

use std::{fmt, io, process::Stdio, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
    sync::Mutex,
};

const HELPER_PATH: &str = "/usr/libexec/polkit-agent-helper-1";

#[derive(Clone)] // XXX?
#[derive(Debug)]
pub enum AgentMsg {
    Failed,
    Responder(AgentHelperResponder),
    Request(String, bool),
    ShowError(String),
    ShowDebug(String),
    Complete(bool),
}

pub fn agent_helper_subscription(pw_name: &str, cookie: &str) -> iced::Subscription<AgentMsg> {
    // TODO: Avoid clone?
    let mut args = Some((pw_name.to_owned(), cookie.to_owned()));
    let name = format!("agent-helper-{}", cookie);
    iced::subscription::unfold(name, None::<AgentHelper>, move |agent_helper| {
        let args = args.take();
        async move {
            if let Some(mut agent_helper) = agent_helper {
                let msg = agent_helper
                    .next()
                    .await
                    .unwrap_or_else(|err| Some(AgentMsg::Failed));
                (msg, Some(agent_helper))
            } else {
                let (pw_name, cookie) = args.unwrap();
                match AgentHelper::new(&pw_name, &cookie).await {
                    Ok((helper, responder)) => (Some(AgentMsg::Responder(responder)), Some(helper)),
                    Err(err) => (Some(AgentMsg::Failed), None),
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
    async fn new(pw_name: &str, cookie: &str) -> io::Result<(Self, AgentHelperResponder)> {
        let mut child = Command::new(HELPER_PATH)
            .arg(pw_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut responder = AgentHelperResponder {
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

    async fn next(&mut self) -> io::Result<Option<AgentMsg>> {
        let mut line = String::new();
        while self.stdout.read_line(&mut line).await? != 0 {
            let line = line.trim();
            let (prefix, rest) = line.split_once(' ').unwrap_or((line, ""));
            return Ok(Some(match prefix {
                "PAM_PROMPT_ECHO_OFF" => AgentMsg::Request(rest.to_string(), false),
                "PAM_PROMPT_ECHO_ON" => AgentMsg::Request(rest.to_string(), true),
                "PAM_ERROR_MSG" => AgentMsg::ShowError(rest.to_string()),
                "PAM_TEXT_INFO" => AgentMsg::ShowDebug(rest.to_string()),
                "SUCCESS" => AgentMsg::Complete(true),
                "FAILURE" => AgentMsg::Complete(false),
                _ => {
                    eprintln!("Unknown line '{}' from 'polkit-agent-helper-1'", line);
                    continue;
                }
            }));
        }
        Ok(None)
    }
}

#[derive(Clone)]
pub struct AgentHelperResponder {
    stdin: Arc<Mutex<tokio::process::ChildStdin>>,
}

impl AgentHelperResponder {
    pub async fn response(&self, resp: &str) -> io::Result<()> {
        let mut stdin = self.stdin.lock().await;
        stdin.write(resp.as_bytes()).await?;
        stdin.write(b"\n").await?;
        Ok(())
    }
}

impl fmt::Debug for AgentHelper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AgentHelper")
    }
}

impl fmt::Debug for AgentHelperResponder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AgentHelperResponder")
    }
}
