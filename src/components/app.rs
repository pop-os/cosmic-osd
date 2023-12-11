use cosmic::{
    iced::{self, Application, Command, Subscription},
    iced_runtime::window::Id as SurfaceId,
    iced_sctk::settings::InitialSurface,
    iced_style::application,
};
use std::collections::HashMap;

use crate::{
    components::polkit_dialog,
    subscriptions::{dbus, polkit_agent, settings_daemon},
};

#[derive(Debug)]
pub enum Msg {
    DBus(dbus::Event),
    PolkitAgent(polkit_agent::Event),
    PolkitDialog((SurfaceId, polkit_dialog::Msg)),
    SettingsDaemon(settings_daemon::Event),
}

enum Surface {
    PolkitDialog(polkit_dialog::State),
}

#[derive(Default)]
struct App {
    max_surface_id: u128,
    connection: Option<zbus::Connection>,
    system_connection: Option<zbus::Connection>,
    surfaces: HashMap<SurfaceId, Surface>,
}

impl Application for App {
    type Message = Msg;
    type Theme = cosmic::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Msg>) {
        (Self::default(), Command::none())
    }

    fn title(&self, _: SurfaceId) -> String {
        String::from("cosmic-osd")
    }

    fn style(&self) -> <Self::Theme as application::StyleSheet>::Style {
        <Self::Theme as application::StyleSheet>::Style::custom(|theme| application::Appearance {
            background_color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            icon_color: theme.cosmic().on_bg_color().into(),
            text_color: theme.cosmic().on_bg_color().into(),
        })
    }

    fn update(&mut self, message: Msg) -> Command<Msg> {
        match message {
            Msg::DBus(event) => {
                match event {
                    dbus::Event::Connection(connection) => self.connection = Some(connection),
                    dbus::Event::SystemConnection(connection) => {
                        self.system_connection = Some(connection)
                    }
                    dbus::Event::Error(context, err) => {
                        eprintln!("Failed to {}: {}", context, err);
                    }
                }
                iced::Command::none()
            }
            Msg::PolkitAgent(event) => match event {
                polkit_agent::Event::CreateDialog(params) => {
                    println!("create: {}", params.cookie);
                    let id = SurfaceId::unique();
                    let (state, cmd) = polkit_dialog::State::new(id, params);
                    self.surfaces
                        .insert(id.clone(), Surface::PolkitDialog(state));
                    cmd
                }
                polkit_agent::Event::CancelDialog { cookie } => {
                    println!("cancel: {}", cookie);
                    if let Some((id, _)) = self.surfaces.iter().find(|(_id, surface)| {
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
            },
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
            Msg::SettingsDaemon(event) => {
                println!("{:?}", event);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Msg> {
        let mut subscriptions = Vec::new();

        subscriptions.push(dbus::subscription().map(Msg::DBus));

        if let Some(connection) = self.system_connection.clone() {
            subscriptions.push(polkit_agent::subscription(connection).map(Msg::PolkitAgent));
        }

        if let Some(connection) = self.connection.clone() {
            subscriptions.push(settings_daemon::subscription(connection).map(Msg::SettingsDaemon));
        }

        subscriptions.extend(self.surfaces.iter().map(|(id, surface)| match surface {
            Surface::PolkitDialog(state) => state.subscription().with(*id).map(Msg::PolkitDialog),
        }));

        iced::Subscription::batch(subscriptions)
    }

    fn view(&self, id: SurfaceId) -> cosmic::Element<'_, Msg> {
        if let Some(surface) = self.surfaces.get(&id) {
            return match surface {
                Surface::PolkitDialog(state) => {
                    state.view().map(move |msg| Msg::PolkitDialog((id, msg)))
                }
            };
        }
        iced::widget::text("").into() // XXX
    }
}

pub fn main() -> iced::Result {
    App::run(iced::Settings {
        antialiasing: true,
        exit_on_close_request: false,
        initial_surface: InitialSurface::None,
        ..Default::default()
    })
}
