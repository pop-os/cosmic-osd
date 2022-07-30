// TODO: implement
// gtk4::PasswordEntry
// Emit `org.freedesktop.PolicyKit1.Error.Cancelled` if cancelled

use gtk4::prelude::*;
use std::{
    collections::HashMap,
    io::{self, prelude::*},
    process::{self, Command, Stdio},
};
use zbus::zvariant;

use crate::polkit_agent_helper::AgentHelper;

const OBJECT_PATH: &str = "/com/system76/CosmicOsd";

#[derive(serde::Serialize)]
pub struct Subject<'a> {
    subject_kind: &'a str,
    subject_details: HashMap<&'a str, zvariant::Value<'a>>,
}

impl<'a> zvariant::Type for Subject<'a> {
    fn signature() -> zvariant::Signature<'static> {
        unsafe { zvariant::Signature::from_bytes_unchecked(b"(sa{sv})") }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Identity<'a> {
    identity_kind: &'a str,
    identity_details: HashMap<&'a str, zvariant::Value<'a>>,
}

impl<'a> zvariant::Type for Identity<'a> {
    fn signature() -> zvariant::Signature<'static> {
        unsafe { zvariant::Signature::from_bytes_unchecked(b"(sa{sv})") }
    }
}

#[zbus::dbus_proxy(
    default_service = "org.freedesktop.login1",
    interface = "org.freedesktop.login1.Session",
    default_path = "/org/freedesktop/login1/session/auto"
)]
trait LogindSession {
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;
}

#[zbus::dbus_proxy(
    default_service = "org.freedesktop.PolicyKit1",
    interface = "org.freedesktop.PolicyKit1.Authority",
    default_path = "/org/freedesktop/PolicyKit1/Authority"
)]
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
    async fn begin_authentication(
        &self,
        action_id: String,
        message: String,
        icon_name: String,
        details: HashMap<String, String>,
        cookie: String,
        identities: Vec<Identity<'_>>,
    ) -> zbus::fdo::Result<()> {
        // XXX?
        println!("Begin auth");

        let mut users = Vec::new();
        for ident in identities {
            if ident.identity_kind == "unix-user" {
                if let Some(zvariant::Value::U32(uid)) = ident.identity_details.get("uid") {
                    if let Some(user) = users::get_user_by_uid(*uid) {
                        if let Some(name) = user.name().to_str() {
                            users.push((*uid, name.to_string()));
                        }
                    }
                }
            }
            // `unix-group` is apparently a thing too, but Gnome Shell doesn't seem to handle it...
        }

        // Like Gnome Shell, try own uid, then root, then first UID in `identities`
        if let Some((_uid, name)) = users
            .iter()
            .find(|(uid, _)| *uid == users::get_current_uid())
            .or(users.iter().find(|(uid, _)| *uid == 0))
            .or_else(|| users.first())
        {
            eprintln!("Name: {}", name);
            // let dialog = gtk4::Dialog::new();
            // dialog.show();
            agent_helper(name, &cookie).await;
        }

        Ok(())
    }
    fn cancel_authentication(&self, cookie: String) -> zbus::fdo::Result<()> {
        Ok(())
    }
}

pub async fn register_agent(system_connection: &zbus::Connection) -> zbus::Result<()> {
    system_connection
        .object_server()
        .at(OBJECT_PATH, PolkitAgent)
        .await?;

    let session = LogindSessionProxy::new(system_connection).await?;
    let session_id = session.id().await?;

    let mut subject_details = HashMap::new();
    subject_details.insert("session-id", session_id.into());
    let subject = Subject {
        subject_kind: "unix-session",
        subject_details,
    };

    // XXX locale
    let authority = PolkitAuthorityProxy::new(system_connection).await?;
    authority
        .register_authentication_agent(subject, "en_US", OBJECT_PATH)
        .await?;
    Ok(())
}

async fn agent_helper(pw_name: &str, cookie: &str) -> io::Result<()> {
    let mut helper = AgentHelper::new(pw_name, cookie).await?;
    while let Some(msg) = helper.next().await? {
        println!("{:?}", msg);
    }
    Ok(())
}
