// If the way this handes surface/window is awkward, could inform design of multi-window in iced

#![allow(clippy::single_match)]

use crate::{
    fl,
    subscriptions::{polkit_agent::PolkitError, polkit_agent_helper},
};
use cosmic::{
    iced::{
        self, Subscription, Task,
        event::{PlatformSpecific, wayland},
        window::Id as SurfaceId,
    },
    iced_runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
    iced_winit::wayland::commands::layer_surface::{
        KeyboardInteractivity, Layer, destroy_layer_surface, get_layer_surface,
    },
    widget,
};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};
use tokio::sync::oneshot;

pub static POLKIT_DIALOG_ID: LazyLock<widget::Id> =
    LazyLock::new(|| widget::Id::new("polkit-dialog".to_string()));

#[derive(Clone, Debug)]
pub struct Params {
    pub pw_name: String,
    pub action_id: String,
    pub message: String,
    pub icon_name: Option<String>,
    pub details: HashMap<String, String>,
    pub cookie: String,
    // XXX `Clone` bound is awkward here
    pub response_sender: Arc<Mutex<Option<oneshot::Sender<Result<(), PolkitError>>>>>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Layer(wayland::LayerEvent),
    Agent(polkit_agent_helper::Event),
    Authenticate,
    Cancel,
    Password(String),
    TogglePasswordVisibility,
}

pub struct State {
    id: SurfaceId,
    pub params: Params,
    responder: Option<polkit_agent_helper::Responder>,
    password: String,
    password_visible: bool,
    message: Option<String>, // TODO show
    password_label: String,  // TODO
    echo: bool,
    text_input_id: iced::id::Id,
    sensitive: bool,
    retries: u32,
    // TODO: Better way to use fluent with iced?
    msg_cancel: String,
    msg_authenticate: String,
    msg_authentication_required: String,
    msg_invalid_password: String,
}

impl State {
    pub fn new<T: 'static>(id: SurfaceId, params: Params) -> (Self, Task<T>) {
        let text_input_id = iced::id::Id::unique();
        let cmd = get_layer_surface(SctkLayerSurfaceSettings {
            id,
            keyboard_interactivity: KeyboardInteractivity::Exclusive,
            namespace: "osd".into(),
            layer: Layer::Overlay,
            size: None,
            ..Default::default()
        });
        (
            Self {
                id,
                params,
                responder: None,
                password: String::new(),
                password_visible: false,
                message: None,
                password_label: String::new(),
                echo: false,
                text_input_id,
                sensitive: true,
                retries: 0,
                msg_cancel: fl!("cancel"),
                msg_authenticate: fl!("authenticate"),
                msg_authentication_required: fl!("authentication-required"),
                msg_invalid_password: fl!("invalid-password"),
            },
            cmd,
        )
    }

    pub fn cancel<T>(self) -> Task<T> {
        self.respond(Err(PolkitError::Cancelled))
    }

    fn respond<T>(self, res: Result<(), PolkitError>) -> Task<T> {
        let sender = self.params.response_sender.lock().unwrap().take().unwrap();
        let _ = sender.send(res);
        destroy_layer_surface(self.id)
    }

    pub fn update(mut self, event: Msg) -> (Option<Self>, Task<Msg>) {
        match event {
            // XXX which layer?
            Msg::Layer(layer_event) => match layer_event {
                wayland::LayerEvent::Focused => {
                    let cmd = widget::text_input::focus(self.text_input_id.clone());
                    return (Some(self), cmd);
                }
                _ => {}
            },
            Msg::Agent(agent_msg) => match agent_msg {
                polkit_agent_helper::Event::Responder(responder) => {
                    self.responder = Some(responder);
                }
                polkit_agent_helper::Event::Failed => {
                    return (None, self.respond(Err(PolkitError::Failed)));
                }
                polkit_agent_helper::Event::Request(s, echo) => {
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
                    if success {
                        return (None, self.respond(Ok(())));
                    } else {
                        self.retries += 1;
                        self.sensitive = true;
                        self.password.clear();
                        let cmd = widget::text_input::focus(self.text_input_id.clone());
                        return (Some(self), cmd);
                        //Err(PolkitError::Failed)
                    };
                }
            },
            Msg::Authenticate => {
                self.sensitive = false; // TODO: show spinner?
                if let Some(responder) = self.responder.clone() {
                    let password = self.password.clone();
                    tokio::spawn(async move { responder.response(&password).await });
                }
            }
            Msg::Cancel => return (None, self.cancel()),
            Msg::Password(password) => {
                self.password = password;
            }
            Msg::TogglePasswordVisibility => {
                self.password_visible = !self.password_visible;
            }
        }
        (Some(self), Task::none())
    }

    pub fn view(&self) -> cosmic::Element<'_, Msg> {
        // TODO Allocates on every keypress?

        let placeholder = self.password_label.trim_end_matches(':');
        let mut password_input = if !self.echo {
            widget::secure_input(
                placeholder,
                &self.password,
                Some(Msg::TogglePasswordVisibility),
                !self.password_visible,
            )
            .id(self.text_input_id.clone())
        } else {
            widget::text_input(placeholder, &self.password).id(self.text_input_id.clone())
        };
        let mut cancel_button = widget::button::standard(&self.msg_cancel);
        let mut authenticate_button = widget::button::suggested(&self.msg_authenticate);
        if self.sensitive {
            password_input = password_input
                .on_input(Msg::Password)
                .on_submit(|_| Msg::Authenticate);
            cancel_button = cancel_button.on_press(Msg::Cancel);
            authenticate_button = authenticate_button.on_press(Msg::Authenticate);
        }
        let mut right_column: Vec<cosmic::Element<_>> = vec![password_input.into()];
        if self.retries > 0 {
            right_column.push(
                widget::text::body(&self.msg_invalid_password)
                    .class(cosmic::theme::Text::Color(iced::Color::from_rgb(
                        1.0, 0.0, 0.0,
                    )))
                    .into(),
            );
        } else {
            right_column.push(widget::text::body("").into())
        }
        let icon = widget::icon::from_name(
            self.params
                .icon_name
                .as_deref()
                .unwrap_or("dialog-authentication"),
        )
        .size(64);
        widget::autosize::autosize(
            widget::dialog::dialog()
                .title(&self.msg_authentication_required)
                .body(&self.params.message)
                .control(widget::column::with_children(right_column).spacing(4))
                .icon(icon)
                .primary_action(authenticate_button)
                .secondary_action(cancel_button),
            POLKIT_DIALOG_ID.clone(),
        )
        .min_width(1.)
        .min_height(1.)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        iced::Subscription::batch([
            iced::event::listen_with(|e, _status, _id| match e {
                iced::Event::PlatformSpecific(PlatformSpecific::Wayland(
                    wayland::Event::Layer(e, ..),
                )) => Some(Msg::Layer(e)),
                _ => None,
            }),
            polkit_agent_helper::subscription(
                &self.params.pw_name,
                &self.params.cookie,
                self.retries,
            )
            .map(Msg::Agent),
        ])
    }
}
