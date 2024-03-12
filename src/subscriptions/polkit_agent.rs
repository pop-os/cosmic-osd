// TODO: only open one dialog at a time?

use cosmic::iced::{self, futures::FutureExt};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::ReceiverStream;
use zbus::zvariant;

use crate::components::polkit_dialog;

const OBJECT_PATH: &str = "/com/system76/CosmicOsd";

pub fn subscription(system_connection: zbus::Connection) -> iced::Subscription<Event> {
    iced::subscription::run_with_id(
        "dbus-polkit-agent",
        async move {
            let (sender, receiver) = mpsc::channel(32);
            tokio::spawn(async move {
                // XXX unwrap
                register_agent(&system_connection, sender).await.unwrap();
            });
            ReceiverStream::new(receiver)
        }
        .flatten_stream(),
    )
}

#[derive(Debug)]
pub enum Event {
    CreateDialog(polkit_dialog::Params),
    CancelDialog { cookie: String },
}

#[allow(dead_code)]
#[derive(Clone, Debug, zbus::DBusError)]
#[zbus(prefix = "org.freedesktop.PolicyKit1.Error")]
pub enum PolkitError {
    Failed,
    Cancelled,
    NotSupported,
    NotAuthorized,
    CancellationIdNotUnique,
}

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

#[zbus::proxy(
    default_service = "org.freedesktop.login1",
    interface = "org.freedesktop.login1.Session",
    default_path = "/org/freedesktop/login1/session/auto"
)]
trait LogindSession {
    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;
}

#[zbus::proxy(
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
}

struct PolkitAgent {
    sender: mpsc::Sender<Event>,
}

#[zbus::interface(name = "org.freedesktop.PolicyKit1.AuthenticationAgent")]
impl PolkitAgent {
    async fn begin_authentication(
        &self,
        action_id: String,
        message: String,
        icon_name: String,
        details: HashMap<String, String>,
        cookie: String,
        identities: Vec<Identity<'_>>,
    ) -> Result<(), PolkitError> {
        if let Some((_uid, pw_name)) = select_user_from_identities(&identities) {
            let (response_sender, response_receiver) = oneshot::channel();
            let icon_name = if !icon_name.is_empty() {
                Some(icon_name)
            } else {
                None
            };
            let _ = self
                .sender
                .send(Event::CreateDialog(polkit_dialog::Params {
                    pw_name,
                    action_id,
                    message,
                    icon_name,
                    details,
                    cookie,
                    response_sender,
                }))
                .await;
            response_receiver.await.unwrap()
        } else {
            Err(PolkitError::Failed)
        }
    }

    async fn cancel_authentication(&self, cookie: String) -> Result<(), PolkitError> {
        let _ = self.sender.send(Event::CancelDialog { cookie }).await;
        Ok(())
    }
}

fn select_user_from_identities(identities: &[Identity]) -> Option<(u32, String)> {
    let mut uids = Vec::new();
    for ident in identities {
        if ident.identity_kind == "unix-user" {
            if let Some(zvariant::Value::U32(uid)) = ident.identity_details.get("uid") {
                uids.push(*uid);
            }
        }
        // `unix-group` is apparently a thing too, but Gnome Shell doesn't seem to handle it...
    }

    // Like Gnome Shell, try own uid, then root, then first UID in `identities`
    let uid = *uids
        .iter()
        .find(|uid| **uid == uzers::get_current_uid())
        .or(uids.iter().find(|uid| **uid == 0))
        .or_else(|| uids.first())?;

    let user = uzers::get_user_by_uid(uid)?;
    Some((uid, user.name().to_str()?.to_string()))
}

async fn register_agent(
    system_connection: &zbus::Connection,
    sender: mpsc::Sender<Event>,
) -> zbus::Result<()> {
    let agent = PolkitAgent { sender };
    system_connection
        .object_server()
        .at(OBJECT_PATH, agent)
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
