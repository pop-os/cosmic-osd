use cosmic::iced_native::window::Id as SurfaceId;
use futures::FutureExt;
use iced::{Application, Command, Element, Subscription};
use iced_sctk::command::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;
use iced_sctk::{
    application::SurfaceIdWrapper, commands::layer_surface::destroy_layer_surface,
    settings::InitialSurface,
};
use sctk::shell::layer::{KeyboardInteractivity, Layer};
use std::collections::{BTreeMap, HashMap};
use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::ReceiverStream;

use super::polkit_dialog;
use crate::{
    polkit_agent::{self, PolkitError},
    polkit_agent_helper::{AgentHelperResponder, AgentMsg},
    settings_daemon,
};

#[derive(Debug)]
pub struct PolkitDialogParams {
    pub pw_name: String,
    pub action_id: String,
    pub message: String,
    pub icon_name: String,
    pub details: HashMap<String, String>,
    pub cookie: String,
    pub response_sender: oneshot::Sender<Result<(), PolkitError>>,
}

#[derive(Debug)]
pub enum Msg {
    CreatePolkitDialog(PolkitDialogParams),
    CancelPolkitDialog { cookie: String },
    PolkitDialog((SurfaceId, polkit_dialog::Msg)),
    Closed(SurfaceIdWrapper),
}

enum Surface {
    PolkitDialog(polkit_dialog::State),
}

#[derive(Default)]
struct App {
    surfaces: BTreeMap<SurfaceId, Surface>,
}

impl App {
    // Get lowest unused ID
    // XXX way hashing is used in iced here may not be ideal
    fn next_surface_id(&self) -> SurfaceId {
        let mut id = 1;
        for i in self.surfaces.keys() {
            if *i == SurfaceId::new(id) {
                id += 1;
            } else {
                break;
            }
        }
        SurfaceId::new(id)
    }
}

impl Application for App {
    type Message = Msg;
    type Theme = cosmic::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Msg>) {
        (Self::default(), destroy_layer_surface(SurfaceId::new(0)))
    }

    fn title(&self) -> String {
        String::from("cosmic-osd")
    }

    fn update(&mut self, message: Msg) -> Command<Msg> {
        match message {
            Msg::CreatePolkitDialog(params) => {
                println!("create: {}", params.cookie);
                // TODO open surface
                let id = self.next_surface_id();
                let (state, cmd) = polkit_dialog::State::new(id, params);
                self.surfaces
                    .insert(id.clone(), Surface::PolkitDialog(state));
                cmd
            }
            Msg::CancelPolkitDialog { cookie } => {
                println!("cancel: {}", cookie);
                if let Some((id, _)) = self.surfaces.iter().find(|(id, surface)| {
                    if let Surface::PolkitDialog(state) = surface {
                        state.params.cookie == cookie
                    } else {
                        false
                    }
                }) {
                    let id = *id;
                    if let Surface::PolkitDialog(state) = self.surfaces.remove(&id).unwrap() {
                        state.cancel()
                    } else {
                        unreachable!()
                    }
                } else {
                    Command::none()
                }
            }
            Msg::Closed(surface) => Command::none(),
            Msg::PolkitDialog((id, msg)) => {
                if let Some(surface) = self.surfaces.remove(&id) {
                    if let Surface::PolkitDialog(state) = surface {
                        let (state, cmd) = state.update(msg);
                        if let Some(state) = state {
                            self.surfaces.insert(id, Surface::PolkitDialog(state));
                        }
                        return cmd.map(move |msg| Msg::PolkitDialog((id, msg)));
                    }
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Msg> {
        let mut subscriptions = vec![];

        subscriptions.push(iced::subscription::run(
            "dbus-service",
            async move {
                let (sender, receiver) = mpsc::channel(32);
                tokio::spawn(async move {
                    dbus_serve(sender).await.unwrap();
                });
                ReceiverStream::new(receiver)
            }
            .flatten_stream(),
        ));

        for (id, surface) in &self.surfaces {
            if let Surface::PolkitDialog(state) = surface {
                subscriptions.push(state.subscription().with(*id).map(Msg::PolkitDialog));
            }
        }

        iced::Subscription::batch(subscriptions)
    }

    fn view(&self, surface: SurfaceIdWrapper) -> Element<'_, Msg, iced::Renderer<Self::Theme>> {
        if let SurfaceIdWrapper::LayerSurface(id) = surface {
            if let Some(surface) = self.surfaces.get(&id) {
                println!("FOO");
                return match surface {
                    Surface::PolkitDialog(state) => {
                        state.view().map(move |msg| Msg::PolkitDialog((id, msg)))
                    }
                };
            }
        }
        iced::widget::text("").into() // XXX
    }

    // TODO: Should be Option<Msg>?
    fn close_requested(&self, surface: SurfaceIdWrapper) -> Msg {
        Msg::Closed(surface)
    }
}

pub fn main() -> iced::Result {
    App::run(iced::Settings {
        antialiasing: true,
        exit_on_close_request: false,
        // XXX no initial surface?
        initial_surface: InitialSurface::LayerSurface(SctkLayerSurfaceSettings {
            keyboard_interactivity: KeyboardInteractivity::None,
            namespace: "ignore".into(),
            size: (Some(1), Some(1)),
            layer: Layer::Background,
            ..Default::default()
        }),
        ..Default::default()
    })
}

async fn dbus_serve(sender: mpsc::Sender<Msg>) -> zbus::Result<()> {
    let system_connection = zbus::ConnectionBuilder::system()?
        .internal_executor(false)
        .build()
        .await?;
    let connection = zbus::ConnectionBuilder::session()?
        .internal_executor(false)
        .build()
        .await?;

    connection.request_name("com.system76.CosmicOsd").await?;
    polkit_agent::register_agent(&system_connection, sender).await?;
    settings_daemon::monitor(&connection).await?;
    Ok(())
}
