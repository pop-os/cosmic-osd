#![allow(irrefutable_let_patterns)]

use cosmic::iced::{
    self,
    event::{self, listen_with, wayland::OverlapNotifyEvent},
    window::Id as SurfaceId,
    Point, Rectangle, Size, Subscription, Task,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    components::{osd_indicator, polkit_dialog},
    subscriptions::{dbus, polkit_agent},
};
use cosmic_settings_subscriptions::{
    airplane_mode, pulse, settings_daemon,
    upower::kbdbacklight::{kbd_backlight_subscription, KeyboardBacklightUpdate},
};

#[derive(Clone, Debug)]
pub enum Msg {
    DBus(dbus::Event),
    PolkitAgent(polkit_agent::Event),
    PolkitDialog((SurfaceId, polkit_dialog::Msg)),
    SettingsDaemon(settings_daemon::Event),
    Pulse(pulse::Event),
    OsdIndicator(osd_indicator::Msg),
    AirplaneMode(bool),
    KeyboardBacklight(KeyboardBacklightUpdate),
    Overlap(OverlapNotifyEvent),
    Size(Size),
}

enum Surface {
    PolkitDialog(polkit_dialog::State),
}

struct App {
    core: cosmic::app::Core,
    connection: Option<zbus::Connection>,
    system_connection: Option<zbus::Connection>,
    surfaces: HashMap<SurfaceId, Surface>,
    indicator: Option<(SurfaceId, osd_indicator::State)>,
    max_display_brightness: Option<i32>,
    display_brightness: Option<i32>,
    max_keyboard_brightness: Option<i32>,
    keyboard_brightness: Option<i32>,
    sink_last_playback: Instant,
    sink_mute: Option<bool>,
    sink_volume: Option<u32>,
    source_mute: Option<bool>,
    source_volume: Option<u32>,
    airplane_mode: Option<bool>,
    overlap: HashMap<String, Rectangle>,
    size: Option<Size>,
}

impl App {
    fn create_indicator(&mut self, params: osd_indicator::Params) -> cosmic::app::Task<Msg> {
        if let Some((_id, ref mut state)) = &mut self.indicator {
            state.replace_params(params)
        } else {
            let id = SurfaceId::unique();
            self.overlap.clear();
            let (state, cmd) = osd_indicator::State::new(id, params);
            self.indicator = Some((id, state));
            cmd
        }
        .map(|x| cosmic::app::Message::App(Msg::OsdIndicator(x)))
    }

    fn handle_overlap(&mut self) {
        let Some((_, state)) = self.indicator.as_mut() else {
            return;
        };
        let Some((bl, br, tl, tr)) = self.size.as_ref().map(|s| {
            (
                Rectangle::new(
                    Point::new(0., s.height / 2.),
                    Size::new(s.width / 2., s.height / 2.),
                ),
                Rectangle::new(
                    Point::new(s.width / 2., s.height / 2.),
                    Size::new(s.width / 2., s.height / 2.),
                ),
                Rectangle::new(Point::new(0., 0.), Size::new(s.width / 2., s.height / 2.)),
                Rectangle::new(
                    Point::new(s.width / 2., 0.),
                    Size::new(s.width / 2., s.height / 2.),
                ),
            )
        }) else {
            return;
        };

        let (mut top, mut left, mut bottom, mut right) = (0, 0, 48, 0);
        for overlap in self.overlap.values() {
            let tl = tl.intersects(overlap);
            let tr = tr.intersects(overlap);
            let bl = bl.intersects(overlap);
            let br = br.intersects(overlap);
            if bl && br {
                bottom += overlap.height as i32;
                continue;
            }
            if tl && tr {
                top += overlap.height as i32;
                continue;
            }
            if tl && bl {
                left += overlap.width as i32;
                continue;
            }
            if tr && br {
                right += overlap.width as i32;
                continue;
            }
        }
        state.margin = (top, right, bottom, left);
    }
}

impl cosmic::Application for App {
    type Message = Msg;
    type Executor = iced::executor::Default;
    type Flags = ();
    const APP_ID: &'static str = "com.system76.CosmicWorkspaces";

    fn init(core: cosmic::app::Core, _flags: ()) -> (Self, cosmic::app::Task<Msg>) {
        (
            Self {
                core,
                connection: None,
                system_connection: None,
                surfaces: HashMap::new(),
                indicator: None,
                display_brightness: None,
                max_display_brightness: None,
                keyboard_brightness: None,
                max_keyboard_brightness: None,
                sink_last_playback: Instant::now(),
                sink_mute: None,
                sink_volume: None,
                source_mute: None,
                source_volume: None,
                airplane_mode: None,
                overlap: HashMap::new(),
                size: None,
            },
            Task::none(),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn update(&mut self, message: Msg) -> cosmic::app::Task<Msg> {
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
                iced::Task::none()
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
                        Task::none()
                    }
                }
            },
            Msg::PolkitDialog((id, msg)) => {
                if let Some(Surface::PolkitDialog(state)) = self.surfaces.remove(&id) {
                    let (state, cmd) = state.update(msg);
                    if let Some(state) = state {
                        self.surfaces.insert(id, Surface::PolkitDialog(state));
                    }
                    return cmd
                        .map(move |msg| cosmic::app::Message::App(Msg::PolkitDialog((id, msg))));
                }
                Task::none()
            }
            Msg::OsdIndicator(msg) => {
                if let Some((id, state)) = self.indicator.take() {
                    let (state, cmd) = state.update(msg);
                    if let Some(state) = state {
                        self.indicator = Some((id, state));
                    }
                    cmd.map(|x| cosmic::app::Message::App(Msg::OsdIndicator(x)))
                } else {
                    Task::none()
                }
            }
            Msg::SettingsDaemon(settings_daemon::Event::Sender(_)) => Task::none(),
            Msg::SettingsDaemon(settings_daemon::Event::MaxDisplayBrightness(max_brightness)) => {
                self.max_display_brightness = Some(max_brightness);
                Task::none()
            }
            Msg::SettingsDaemon(settings_daemon::Event::DisplayBrightness(brightness)) => {
                if self.display_brightness.is_none() {
                    self.display_brightness = Some(brightness);
                    Task::none()
                } else if self.display_brightness != Some(brightness) {
                    self.display_brightness = Some(brightness);
                    self.create_indicator(osd_indicator::Params::DisplayBrightness(
                        brightness as f64 / self.max_display_brightness.unwrap_or(100) as f64,
                    ))
                } else {
                    Task::none()
                }
            }
            Msg::Pulse(evt) => {
                match evt {
                    pulse::Event::SinkMute(mute) => {
                        if self.sink_mute.is_none() {
                            self.sink_mute = Some(mute);
                        } else if self.sink_mute != Some(mute) {
                            self.sink_mute = Some(mute);
                            if let Some(sink_volume) = self.sink_volume {
                                return self.create_indicator(osd_indicator::Params::SinkVolume(
                                    sink_volume,
                                    mute,
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
                            if let Some(mute) = self.sink_mute {
                                return self.create_indicator(osd_indicator::Params::SinkVolume(
                                    volume, mute,
                                ));
                            }
                        }
                    }
                    pulse::Event::SourceMute(mute) => {
                        if self.source_mute.is_none() {
                            self.source_mute = Some(mute);
                        } else if self.source_mute != Some(mute) {
                            self.source_mute = Some(mute);
                            if let Some(source_volume) = self.source_volume {
                                return self.create_indicator(osd_indicator::Params::SourceVolume(
                                    source_volume,
                                    mute,
                                ));
                            }
                        }
                    }
                    pulse::Event::SourceVolume(volume) => {
                        if self.source_volume.is_none() {
                            self.source_volume = Some(volume);
                        } else if self.source_volume != Some(volume) {
                            self.source_volume = Some(volume);
                            if let Some(mute) = self.source_mute {
                                return self.create_indicator(osd_indicator::Params::SourceVolume(
                                    volume, mute,
                                ));
                            }
                        }
                    }
                    pulse::Event::CardInfo(_) => {}
                    pulse::Event::DefaultSink(_) => {}
                    pulse::Event::DefaultSource(_) => {}
                }
                Task::none()
            }
            Msg::AirplaneMode(state) => {
                if self.airplane_mode.is_none() {
                    self.airplane_mode = Some(state);
                } else if self.airplane_mode != Some(state) {
                    self.airplane_mode = Some(state);
                    return self.create_indicator(osd_indicator::Params::AirplaneMode(state));
                }
                Task::none()
            }
            Msg::KeyboardBacklight(update) => match update {
                KeyboardBacklightUpdate::Sender(_) => Task::none(),
                KeyboardBacklightUpdate::MaxBrightness(max_brightness) => {
                    self.max_keyboard_brightness = Some(max_brightness);
                    Task::none()
                }
                KeyboardBacklightUpdate::Brightness(brightness) => {
                    if self.keyboard_brightness.is_none() {
                        self.keyboard_brightness = Some(brightness);
                        Task::none()
                    } else if self.keyboard_brightness != Some(brightness) {
                        self.keyboard_brightness = Some(brightness);
                        if let Some(max_brightness) = self.max_keyboard_brightness {
                            self.create_indicator(osd_indicator::Params::KeyboardBrightness(
                                brightness as f64 / max_brightness as f64,
                            ))
                        } else {
                            Task::none()
                        }
                    } else {
                        Task::none()
                    }
                }
            },
            Msg::Overlap(overlap_notify_event) => {
                match overlap_notify_event {
                    OverlapNotifyEvent::OverlapLayerAdd {
                        identifier,
                        namespace,
                        logical_rect,
                        exclusive,
                        ..
                    } => {
                        if namespace == "Dock" || namespace == "Panel" || exclusive > 0 {
                            self.overlap.insert(identifier, logical_rect);
                            self.handle_overlap();
                        }
                    }
                    OverlapNotifyEvent::OverlapLayerRemove { identifier } => {
                        if self.overlap.remove(&identifier).is_some() {
                            self.handle_overlap();
                        }
                    }
                    _ => {}
                }
                Task::none()
            }
            Msg::Size(size) => {
                self.size = Some(size);
                self.handle_overlap();
                Task::none()
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

        subscriptions.push(kbd_backlight_subscription("kbd-backlight").map(Msg::KeyboardBacklight));

        subscriptions.push(listen_with(|event, _, _id| match event {
            event::Event::Window(iced::window::Event::Opened { position: _, size }) => {
                Some(Msg::Size(size))
            }
            event::Event::Window(iced::window::Event::Resized(s)) => Some(Msg::Size(s)),
            event::Event::PlatformSpecific(event::PlatformSpecific::Wayland(wayland_event)) => {
                match wayland_event {
                    event::wayland::Event::OverlapNotify(event) => Some(Msg::Overlap(event)),
                    _ => None,
                }
            }
            _ => None,
        }));

        subscriptions.extend(self.surfaces.iter().map(|(id, surface)| match surface {
            Surface::PolkitDialog(state) => state.subscription().with(*id).map(Msg::PolkitDialog),
        }));

        iced::Subscription::batch(subscriptions)
    }

    fn view(&self) -> cosmic::prelude::Element<Self::Message> {
        unreachable!()
    }

    fn view_window(&self, id: SurfaceId) -> cosmic::Element<'_, Msg> {
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
    cosmic::app::run::<App>(
        cosmic::app::Settings::default()
            .no_main_window(true)
            .exit_on_close(false),
        (),
    )
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
