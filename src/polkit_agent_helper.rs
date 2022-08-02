use async_process::{self, Command, Stdio};
use futures::{io::BufReader, prelude::*};
use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

const HELPER_PATH: &str = "/usr/libexec/polkit-agent-helper-1";

#[derive(Debug)]
pub enum AgentMsg {
    Request(String, bool),
    ShowError(String),
    ShowDebug(String),
    Complete(bool),
}

pub struct AgentHelper {
    child: async_process::Child,
    stdout: BufReader<async_process::ChildStdout>,
}

impl AgentHelper {
    pub async fn new(pw_name: &str, cookie: &str) -> io::Result<(Self, AgentHelperResponder)> {
        let mut child = Command::new(HELPER_PATH)
            .arg(pw_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut responder = AgentHelperResponder {
            stdin: child.stdin.take().unwrap(),
        };
        responder.response(cookie).await?;
        Ok((
            Self {
                stdout: BufReader::new(child.stdout.take().unwrap()),
                child,
            },
            responder,
        ))
    }

    pub async fn next(&mut self) -> io::Result<Option<AgentMsg>> {
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

pub struct AgentHelperResponder {
    stdin: async_process::ChildStdin,
}

impl AgentHelperResponder {
    pub async fn response(&mut self, resp: &str) -> io::Result<()> {
        self.stdin.write(resp.as_bytes()).await?;
        self.stdin.write(b"\n").await?;
        Ok(())
    }
}
