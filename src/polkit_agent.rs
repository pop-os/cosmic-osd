// TODO: implement
// gtk4::PasswordEntry

use std::{
    collections::HashMap,
    io::{self, prelude::*},
    process::{Command, Stdio},
};
use zbus::zvariant;

#[derive(serde::Serialize)]
struct Subject<'a> {
    subject_kind: &'a str,
    subject_details: HashMap<&'a str, zvariant::Value<'a>>,
}

impl<'a> zvariant::Type for Subject<'a> {
    fn signature() -> zvariant::Signature<'static> {
        unsafe { zvariant::Signature::from_bytes_unchecked(b"sa{sv}") }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Identity<'a> {
    identity_kind: &'a str,
    identity_details: HashMap<&'a str, zvariant::Value<'a>>,
}

impl<'a> zvariant::Type for Identity<'a> {
    fn signature() -> zvariant::Signature<'static> {
        unsafe { zvariant::Signature::from_bytes_unchecked(b"sa{sv}") }
    }
}

#[zbus::dbus_proxy]
trait PolkitAuthority {
    fn register_authentication_agent(
        &self,
        subject: Subject<'_>,
        locale: &str,
        object_path: &str,
    ) -> zbus::Result<()>;
    fn unregister_authentication_agent(
        &self,
        subject: Subject<'_>,
        object_path: &str,
    ) -> zbus::Result<()>;
    fn authentication_agent_response(
        &self,
        cookie: &str,
        identity: Identity<'_>,
    ) -> zbus::Result<()>;
}

struct PolkitAgent;

#[zbus::dbus_interface(name = "org.freedesktop.PolicyKit1.AuthenticationAgent")]
impl PolkitAgent {
    fn begin_authentication(
        &self,
        action_id: String,
        message: String,
        icon_name: String,
        details: HashMap<String, String>,
        cookie: String,
        identities: Vec<Identity>,
    ) -> zbus::fdo::Result<()> {
        Ok(())
    }
    fn cancel_authentication(&self, cookie: String) -> zbus::fdo::Result<()> {
        Ok(())
    }
}

fn request(s: &str, echo: bool) {}

fn show_error(s: &str) {}

fn show_debug(s: &str) {}

fn complete(success: bool) {}

enum AgentMsg<'a> {
    Request(&'a str, bool),
    ShowError(&'a str),
    ShowDebug(&'a str),
    Complete(bool),
}

fn agent_helper(pw_name: &str, cookie: &str) -> io::Result<()> {
    let mut child = Command::new("/usr/libexec/polkit-agent-helper-1")
        .arg(pw_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    let stdout = io::BufReader::new(child.stdout.take().unwrap());
    stdin.write(cookie.as_bytes())?;
    stdin.write(b"\n")?;
    stdin.flush()?;
    for line in stdout.lines() {
        let line = line?;
        let line = line.trim();
        let (prefix, rest) = line.split_once(' ').unwrap_or((line, ""));
        match prefix {
            "PAM_PROMPT_ECHO_OFF" => request(rest, false),
            "PAM_PROMPT_ECHO_ON" => request(rest, true),
            "PAM_ERROR_MSG" => show_error(rest),
            "PAM_TEXT_INFO" => show_debug(rest),
            "SUCCESS" => complete(true),
            "FAILURE" => complete(false),
            _ => eprintln!("Unknown line '{}' from 'polkit-agent-helper-1'", line),
        }
    }
    Ok(())
}

// fn response
// write to stdin
