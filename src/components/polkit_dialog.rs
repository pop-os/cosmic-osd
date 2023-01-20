// TODO translate
// If the way this handles surface/window is awkward, could inform design of multi-window in iced

use cosmic::iced_native::window::Id as SurfaceId;
use cosmic::{theme, Renderer};
use iced::{
    wayland::layer_surface::{KeyboardInteractivity, Layer},
    widget, Command, Element, Subscription,
};
use iced_sctk::{
    command::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
    commands::layer_surface::{destroy_layer_surface, get_layer_surface},
};
use std::collections::HashMap;
use tokio::sync::oneshot;

use crate::subscriptions::{polkit_agent::PolkitError, polkit_agent_helper};

#[derive(Debug)]
pub struct Params {
    pub pw_name: String,
    pub action_id: String,
    pub message: String,
    pub icon_name: Option<String>,
    pub details: HashMap<String, String>,
    pub cookie: String,
    pub response_sender: oneshot::Sender<Result<(), PolkitError>>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    AgentMsg(polkit_agent_helper::Event),
    Authenticate,
    Cancel,
    Password(String),
}

pub struct State {
    id: SurfaceId,
    pub params: Params,
    responder: Option<polkit_agent_helper::Responder>,
    password: String,
    message: Option<String>, // TODO show
    password_label: String,  // TODO
    echo: bool,
}

impl State {
    pub fn new<T>(id: SurfaceId, params: Params) -> (Self, Command<T>) {
        let cmd = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            namespace: "osd".into(),
            layer: Layer::Overlay,
            // XXX size window to fit content?
            size: Some((Some(600), Some(300))),
            ..Default::default()
        });
        (
            Self {
                id,
                params,
                responder: None,
                password: String::new(),
                message: None,
                password_label: String::new(),
                echo: false,
            },
            cmd,
        )
    }

    pub fn cancel<T>(self) -> Command<T> {
        self.respond(Err(PolkitError::Cancelled))
    }

    fn respond<T>(self, res: Result<(), PolkitError>) -> Command<T> {
        let _ = self.params.response_sender.send(res);
        destroy_layer_surface(self.id)
    }

    pub fn update(mut self, event: Msg) -> (Option<Self>, Command<Msg>) {
        match event {
            Msg::AgentMsg(agent_msg) => match agent_msg {
                polkit_agent_helper::Event::Responder(responder) => {
                    self.responder = Some(responder);
                }
                polkit_agent_helper::Event::Failed => {
                    return (None, self.respond(Err(PolkitError::Failed)));
                }
                polkit_agent_helper::Event::Request(s, echo) => {
                    println!("request: {}", s);
                    self.password_label = s;
                    self.echo = echo;
                }
                polkit_agent_helper::Event::ShowError(s) => {
                    self.message = Some(s);
                }
                polkit_agent_helper::Event::ShowDebug(s) => {
                    self.message = Some(s);
                }
                polkit_agent_helper::Event::Complete(success) => {
                    let res = if success {
                        Ok(())
                    } else {
                        Err(PolkitError::Failed)
                    };
                    return (None, self.respond(res));
                }
            },
            Msg::Authenticate => {
                // TODO insenstive until ready?
                if let Some(responder) = self.responder.clone() {
                    let password = self.password.clone();
                    tokio::spawn(async move { responder.response(&password).await });
                }
            }
            Msg::Cancel => return (None, self.cancel()),
            Msg::Password(password) => {
                self.password = password;
            }
        }
        (Some(self), Command::none())
    }

    pub fn view(&self) -> Element<'_, Msg, Renderer> {
        // TODO Allocates on every keypress?
        let mut password_input =
            widget::text_input("", &self.password, |password| Msg::Password(password));
        if !self.echo {
            password_input = password_input.password();
        }
        widget::row![
            cosmic::widget::icon(self.params.icon_name.as_deref().unwrap_or(""), 64), // XXX test if name is empty
            widget::column![
                widget::text("Authentication Required"),
                widget::text(&self.params.message),
                widget::row![widget::text(&self.password_label), password_input,],
                widget::row![
                    cosmic::widget::button(theme::Button::Secondary)
                        .text("Cancel")
                        .on_press(Msg::Cancel),
                    cosmic::widget::button(theme::Button::Primary)
                        .text("Authenticate")
                        .on_press(Msg::Authenticate),
                ]
            ],
        ]
        .into()
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        polkit_agent_helper::subscription(&self.params.pw_name, &self.params.cookie)
            .map(Msg::AgentMsg)
    }
}
