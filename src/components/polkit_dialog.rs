// If the way this handes surface/window is awkward, could inform design of multi-window in iced

#![allow(clippy::single_match)]

use cosmic::{
    iced::{
        self,
        event::{wayland, PlatformSpecific},
        widget, Command, Subscription,
    },
    iced_runtime::{
        command::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
        window::Id as SurfaceId,
    },
    iced_sctk::commands::layer_surface::{
        destroy_layer_surface, get_layer_surface, KeyboardInteractivity, Layer,
    },
    theme,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;

use crate::{
    fl,
    subscriptions::{polkit_agent::PolkitError, polkit_agent_helper},
};

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
}

pub struct State {
    id: SurfaceId,
    pub params: Params,
    responder: Option<polkit_agent_helper::Responder>,
    password: String,
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
    pub fn new<T: 'static>(id: SurfaceId, params: Params) -> (Self, Command<T>) {
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

    pub fn cancel<T>(self) -> Command<T> {
        self.respond(Err(PolkitError::Cancelled))
    }

    fn respond<T>(self, res: Result<(), PolkitError>) -> Command<T> {
        let sender = self.params.response_sender.lock().unwrap().take().unwrap();
        let _ = sender.send(res);
        destroy_layer_surface(self.id)
    }

    pub fn update(mut self, event: Msg) -> (Option<Self>, Command<Msg>) {
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
        }
        (Some(self), Command::none())
    }

    pub fn view(&self) -> cosmic::Element<'_, Msg> {
        // TODO Allocates on every keypress?
        let active_theme = cosmic::theme::active();
        let cosmic_theme = active_theme.cosmic();
        let placeholder = self.password_label.trim_end_matches(':');
        let mut password_input =
            cosmic::widget::text_input(placeholder, &self.password).id(self.text_input_id.clone());
        if !self.echo {
            password_input = password_input.password();
        }
        let mut cancel_button = cosmic::widget::button(min_width_and_height(
            cosmic::widget::text(&self.msg_cancel).size(14).into(),
            142.0,
            32.0,
        ))
        .padding([0, cosmic_theme.space_s()])
        .style(cosmic::theme::Button::Standard);
        let mut authenticate_button = cosmic::widget::button(min_width_and_height(
            cosmic::widget::text(&self.msg_authenticate).size(14).into(),
            142.0,
            32.0,
        ))
        .padding([0, cosmic_theme.space_s()])
        .style(cosmic::theme::Button::Suggested);
        if self.sensitive {
            password_input = password_input
                .on_input(Msg::Password)
                .on_submit(Msg::Authenticate);
            cancel_button = cancel_button.on_press(Msg::Cancel);
            authenticate_button = authenticate_button.on_press(Msg::Authenticate);
        }
        let mut right_column: Vec<cosmic::Element<_>> = vec![
            widget::text(&self.params.message).into(),
            password_input.into(),
        ];
        if self.retries > 0 {
            right_column.push(
                cosmic::widget::text::caption(&self.msg_invalid_password)
                    .style(cosmic::theme::Text::Color(
                        cosmic_theme.destructive_color().into(),
                    ))
                    .into(),
            );
        }
        let icon = cosmic::widget::icon::from_name(
            self.params
                .icon_name
                .as_deref()
                .unwrap_or("dialog-authentication"),
        )
        .size(64);
        cosmic::widget::dialog::dialog(&self.msg_authentication_required)
            .control(widget::column(right_column).spacing(6))
            .icon(icon)
            .primary_action(authenticate_button)
            .secondary_action(cancel_button)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Msg> {
        iced::Subscription::batch([
            cosmic::iced::event::listen_with(|e, _status| match e {
                cosmic::iced::Event::PlatformSpecific(PlatformSpecific::Wayland(
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

fn min_width_and_height<'a>(
    e: cosmic::Element<'a, Msg>,
    width: impl Into<iced::Length>,
    height: impl Into<iced::Length>,
) -> cosmic::widget::Column<'a, Msg> {
    cosmic::widget::column::with_children(vec![
        cosmic::widget::row::with_children(vec![e, cosmic::widget::vertical_space(height).into()])
            .align_items(iced::Alignment::Center)
            .into(),
        cosmic::widget::horizontal_space(width).into(),
    ])
    .align_items(iced::Alignment::Center)
}
