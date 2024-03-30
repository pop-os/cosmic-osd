#![allow(irrefutable_let_patterns)]

use cosmic::{
    iced::{self, Application, Command, Subscription},
    iced_runtime::window::Id as SurfaceId,
    iced_sctk::settings::InitialSurface,
    iced_style::application,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    components::{osd_indicator, polkit_dialog},
    subscriptions::{airplane_mode, dbus, polkit_agent, pulse, settings_daemon},
};

#[derive(Debug)]
pub enum Msg {
    DBus(dbus::Event),
    PolkitAgent(polkit_agent::Event),
    PolkitDialog((SurfaceId, polkit_dialog::Msg)),
    SettingsDaemon(settings_daemon::Event),
    Pulse(pulse::Event),
    OsdIndicator(osd_indicator::Msg),
    AirplaneMode(bool),
}

enum Surface {
    PolkitDialog(polkit_dialog::State),
}

struct App {
    connection: Option<zbus::Connection>,
    system_connection: Option<zbus::Connection>,
    surfaces: HashMap<SurfaceId, Surface>,
    indicator: Option<(SurfaceId, osd_indicator::State)>,
    display_brightness: Option<i32>,
    keyboard_brightness: Option<i32>,
    sink_last_playback: Instant,
    sink_mute: Option<bool>,
    sink_volume: Option<u32>,
    source_mute: Option<bool>,
    source_volume: Option<u32>,
    airplane_mode: Option<bool>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            connection: None,
            system_connection: None,
            surfaces: HashMap::new(),
            indicator: None,
            display_brightness: None,
            keyboard_brightness: None,
            sink_last_playback: Instant::now(),
            sink_mute: None,
            sink_volume: None,
            source_mute: None,
            source_volume: None,
            airplane_mode: None,
        }
    }
}

impl App {
    fn create_indicator(&mut self, params: osd_indicator::Params) -> Command<Msg> {
        if let Some((_id, ref mut state)) = &mut self.indicator {
            state.replace_params(params)
        } else {
            let id = SurfaceId::unique();
            let (state, cmd) = osd_indicator::State::new(id, params);
            self.indicator = Some((id, state));
            cmd
        }
        .map(Msg::OsdIndicator)
    }
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
                        log::error!("Failed to {}: {}", context, err);
                    }
                }
                iced::Command::none()
            }
            Msg::PolkitAgent(event) => match event {
                polkit_agent::Event::CreateDialog(params) => {
                    log::trace!("create polkit dialog: {}", params.cookie);
                    let id = SurfaceId::unique();
                    let (state, cmd) = polkit_dialog::State::new(id, params);
                    self.surfaces.insert(id, Surface::PolkitDialog(state));
                    cmd
                }
                polkit_agent::Event::CancelDialog { cookie } => {
                    log::trace!("cancel polkit dialog: {}", cookie);
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
                if let Some(Surface::PolkitDialog(state)) = self.surfaces.remove(&id) {
                    let (state, cmd) = state.update(msg);
                    if let Some(state) = state {
                        self.surfaces.insert(id, Surface::PolkitDialog(state));
                    }
                    return cmd.map(move |msg| Msg::PolkitDialog((id, msg)));
                }
                Command::none()
            }
            Msg::OsdIndicator(msg) => {
                if let Some((id, state)) = self.indicator.take() {
                    let (state, cmd) = state.update(msg);
                    if let Some(state) = state {
                        self.indicator = Some((id, state));
                    }
                    cmd.map(Msg::OsdIndicator)
                } else {
                    Command::none()
                }
            }
            Msg::SettingsDaemon(settings_daemon::Event::DisplayBrightness(brightness)) => {
                if self.display_brightness.is_none() {
                    self.display_brightness = Some(brightness);
                    Command::none()
                } else if self.display_brightness != Some(brightness) {
                    self.display_brightness = Some(brightness);
                    self.create_indicator(osd_indicator::Params::DisplayBrightness(brightness))
                } else {
                    Command::none()
                }
            }
            Msg::SettingsDaemon(settings_daemon::Event::KeyboardBrightness(brightness)) => {
                if self.keyboard_brightness.is_none() {
                    self.keyboard_brightness = Some(brightness);
                    Command::none()
                } else if self.keyboard_brightness != Some(brightness) {
                    self.keyboard_brightness = Some(brightness);
                    self.create_indicator(osd_indicator::Params::KeyboardBrightness(brightness))
                } else {
                    Command::none()
                }
            }
            Msg::Pulse(evt) => {
                match evt {
                    pulse::Event::SinkMute(mute) => {
                        if self.sink_mute.is_none() {
                            self.sink_mute = Some(mute);
                        } else if self.sink_mute != Some(mute) {
                            self.sink_mute = Some(mute);
                            if mute {
                                return self.create_indicator(osd_indicator::Params::SinkMute);
                            } else if let Some(sink_volume) = self.sink_volume {
                                return self.create_indicator(osd_indicator::Params::SinkVolume(
                                    sink_volume,
                                ));
                            }
                        }
                    }
                    pulse::Event::SinkVolume(volume) => {
                        let now = Instant::now();
                        if now.duration_since(self.sink_last_playback) > Duration::from_millis(125)
                        {
                            self.sink_last_playback = now;
                            pipewire::play_audio_volume_change();
                        }

                        if self.sink_volume.is_none() {
                            self.sink_volume = Some(volume);
                        } else if self.sink_volume != Some(volume) {
                            self.sink_volume = Some(volume);
                            return self
                                .create_indicator(osd_indicator::Params::SinkVolume(volume));
                        }
                    }
                    pulse::Event::SourceMute(mute) => {
                        if self.source_mute.is_none() {
                            self.source_mute = Some(mute);
                        } else if self.source_mute != Some(mute) {
                            self.source_mute = Some(mute);
                            if mute {
                                return self.create_indicator(osd_indicator::Params::SourceMute);
                            } else if let Some(source_volume) = self.source_volume {
                                return self.create_indicator(osd_indicator::Params::SourceVolume(
                                    source_volume,
                                ));
                            }
                        }
                    }
                    pulse::Event::SourceVolume(volume) => {
                        if self.source_volume.is_none() {
                            self.source_volume = Some(volume);
                        } else if self.source_volume != Some(volume) {
                            self.source_volume = Some(volume);
                            return self
                                .create_indicator(osd_indicator::Params::SourceVolume(volume));
                        }
                    }
                }
                Command::none()
            }
            Msg::AirplaneMode(state) => {
                if self.airplane_mode.is_none() {
                    self.airplane_mode = Some(state);
                } else if self.airplane_mode != Some(state) {
                    self.airplane_mode = Some(state);
                    return self.create_indicator(osd_indicator::Params::AirplaneMode(state));
                }
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

        subscriptions.push(pulse::subscription().map(Msg::Pulse));

        subscriptions.push(airplane_mode::subscription().map(Msg::AirplaneMode));

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
        } else if let Some((indicator_id, state)) = &self.indicator {
            if id == *indicator_id {
                return state.view().map(Msg::OsdIndicator);
            }
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

mod pipewire {
    // Copyright 2023 System76 <info@system76.com>
    // SPDX-License-Identifier: MPL-2.0

    use std::path::Path;
    use std::process::Stdio;

    /// Plays an audio file.
    pub fn play(path: &Path) {
        let _result = tokio::process::Command::new("pw-play")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg(path)
            .spawn();
    }

    pub fn play_audio_volume_change() {
        play(Path::new(
            "/usr/share/sounds/freedesktop/stereo/audio-volume-change.oga",
        ));
    }
}
