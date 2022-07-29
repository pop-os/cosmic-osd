use std::{
    collections::HashMap,
    io::{self, prelude::*},
    process::{self, Command, Stdio},
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
    child: process::Child,
    stdin: process::ChildStdin,
    stdout: io::BufReader<process::ChildStdout>,
}

impl AgentHelper {
    pub fn new(pw_name: &str, cookie: &str) -> io::Result<Self> {
        let mut child = Command::new(HELPER_PATH)
            .arg(pw_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut stdin = child.stdin.take().unwrap();
        let stdout = io::BufReader::new(child.stdout.take().unwrap());
        writeln!(stdin, "{}", cookie)?;
        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }

    pub fn next(&mut self) -> io::Result<Option<AgentMsg>> {
        let mut line = String::new();
        while self.stdout.read_line(&mut line)? != 0 {
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

    pub fn response(&mut self, resp: &str) -> io::Result<()> {
        writeln!(self.stdin, "{}", resp)
    }
}
