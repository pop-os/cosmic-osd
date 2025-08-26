#![allow(irrefutable_let_patterns)]

use crate::{
    components::{osd_indicator, polkit_dialog},
    fl,
    subscriptions::{dbus, polkit_agent},
};
use crate::{cosmic_session::CosmicSessionProxy, session_manager::SessionManagerProxy};
use clap::Parser;
use cosmic::{
    Element,
    app::{self, CosmicFlags, Task},
    dbus_activation::Details,
    iced::{
        self, Alignment, Length, Limits, Point, Rectangle, Size, Subscription,
        event::{
            self, listen_with,
            wayland::{self, LayerEvent, OverlapNotifyEvent},
        },
        keyboard::{Key, key::Named},
        time,
        window::Id as SurfaceId,
    },
    iced_runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
    iced_winit::commands::layer_surface::{
        Anchor, KeyboardInteractivity, destroy_layer_surface, get_layer_surface,
    },
    theme,
    widget::{Column, autosize::autosize, button, container, icon, text},
};
use cosmic_settings_subscriptions::{
    airplane_mode, pulse, settings_daemon,
    upower::kbdbacklight::{KeyboardBacklightUpdate, kbd_backlight_subscription},
};
use logind_zbus::manager::ManagerProxy;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    str::FromStr,
    sync::LazyLock,
    time::{Duration, Instant},
};
use zbus::Connection;

const COUNTDOWN_LENGTH: u8 = 60;
static CONFIRM_ID: LazyLock<iced::id::Id> = LazyLock::new(|| iced::id::Id::new("confirm-id"));
static AUTOSIZE_DIALOG_ID: LazyLock<iced::id::Id> =
    LazyLock::new(|| iced::id::Id::new("autosize-id"));

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Copy)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<OsdTask>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, clap::Subcommand)]
pub enum OsdTask {
    #[clap(about = "Toggle the on screen display and start the log out timer")]
    LogOut,
    #[clap(about = "Toggle the on screen display and start the restart timer")]
    Restart,
    #[clap(about = "Toggle the on screen display and start the shutdown timer")]
    Shutdown,
    #[clap(about = "Toggle the on screen display and start the restart to bios timer")]
    EnterBios,
}

impl OsdTask {
    fn perform(self) -> Task<Msg> {
        let msg = |m| cosmic::action::app(Msg::Zbus(m));
        match self {
            OsdTask::EnterBios => cosmic::task::future(restart(true)).map(msg),
            OsdTask::LogOut => cosmic::task::future(log_out()).map(msg),
            OsdTask::Restart => cosmic::task::future(restart(false)).map(msg),
            OsdTask::Shutdown => cosmic::task::future(shutdown()).map(msg),
        }
    }
}

async fn restart(reboot_to_firmware_setup: bool) -> zbus::Result<()> {
    let connection = Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;
    _ = manager_proxy
        .set_reboot_to_firmware_setup(reboot_to_firmware_setup)
        .await;
    manager_proxy.reboot(true).await
}

async fn shutdown() -> zbus::Result<()> {
    let connection = Connection::system().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;
    manager_proxy.power_off(true).await
}

async fn log_out() -> zbus::Result<()> {
    let session_type = std::env::var("XDG_CURRENT_DESKTOP").ok();
    let connection = Connection::session().await?;
    match session_type.as_ref().map(|s| s.trim()) {
        Some("pop:GNOME") => {
            let manager_proxy = SessionManagerProxy::new(&connection).await?;
            manager_proxy.logout(0).await?;
        }
        // By default assume COSMIC
        _ => {
            let cosmic_session = CosmicSessionProxy::new(&connection).await?;
            cosmic_session.exit().await?;
        }
    }
    Ok(())
}

impl Display for OsdTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::ser::to_string(self).unwrap())
    }
}

impl FromStr for OsdTask {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::de::from_str(s)
    }
}

impl CosmicFlags for Args {
    type SubCommand = OsdTask;
    type Args = Vec<String>;

    fn action(&self) -> Option<&OsdTask> {
        self.subcommand.as_ref()
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    Action(OsdTask),
    Confirm,
    Cancel,
    Countdown,
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
    Zbus(Result<(), zbus::Error>),
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
    action_to_confirm: Option<(SurfaceId, OsdTask, u8)>,
}

impl App {
    fn create_indicator(&mut self, params: osd_indicator::Params) -> cosmic::app::Task<Msg> {
        if let Some((_id, state)) = &mut self.indicator {
            state.replace_params(params)
        } else {
            let id = SurfaceId::unique();
            self.overlap.clear();
            let (state, cmd) = osd_indicator::State::new(id, params);
            self.indicator = Some((id, state));
            cmd
        }
        .map(|x| cosmic::Action::App(Msg::OsdIndicator(x)))
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
    type Flags = Args;
    const APP_ID: &'static str = "com.system76.CosmicOnScreenDisplay";

    fn init(core: cosmic::app::Core, _flags: Args) -> (Self, cosmic::app::Task<Msg>) {
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
                action_to_confirm: None,
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

    fn update(&mut self, message: Msg) -> Task<Msg> {
        match message {
            Msg::Action(action) => {
                // Ask for user confirmation of non-destructive actions only
                if matches!(action, OsdTask::Restart)
                    && matches!(self.action_to_confirm, Some((_, OsdTask::Shutdown, _)))
                {
                    action.perform()
                } else {
                    let id = SurfaceId::unique();
                    self.action_to_confirm = Some((id, action, COUNTDOWN_LENGTH));
                    get_layer_surface(SctkLayerSurfaceSettings {
                        id,
                        keyboard_interactivity: KeyboardInteractivity::Exclusive,
                        anchor: Anchor::empty(),
                        namespace: "dialog".into(),
                        size: None,
                        size_limits: Limits::NONE.min_width(1.0).min_height(1.0),
                        ..Default::default()
                    })
                }
            }
            Msg::Confirm => {
                if let Some((id, a, _)) = self.action_to_confirm.take() {
                    app::Task::batch(vec![destroy_layer_surface(id), a.perform()])
                } else {
                    Task::none()
                }
            }
            Msg::Cancel => {
                if let Some((id, _, _)) = self.action_to_confirm.take() {
                    return destroy_layer_surface(id);
                }
                Task::none()
            }
            Msg::Countdown => {
                if let Some((surface_id, a, countdown)) = self.action_to_confirm.as_mut() {
                    *countdown -= 1;
                    if *countdown == 0 {
                        let id = *surface_id;
                        let a = *a;

                        self.action_to_confirm = None;
                        return app::Task::batch(vec![destroy_layer_surface(id), a.perform()]);
                    }
                }
                Task::none()
            }
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
                    return cmd.map(move |msg| cosmic::action::app(Msg::PolkitDialog((id, msg))));
                }
                Task::none()
            }
            Msg::OsdIndicator(msg) => {
                if let Some((id, state)) = self.indicator.take() {
                    let (state, cmd) = state.update(msg);
                    if let Some(state) = state {
                        self.indicator = Some((id, state));
                    }
                    cmd.map(|x| cosmic::action::app(Msg::OsdIndicator(x)))
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
                    pulse::Event::Balance(_) | pulse::Event::Channels(_) => {}
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
            Msg::Zbus(result) => {
                if let Err(e) = result {
                    eprintln!("cosmic-osd ERROR: '{}'", e);
                }
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
                    wayland::Event::Layer(LayerEvent::Unfocused, ..) => Some(Msg::Cancel),
                    _ => None,
                }
            }
            cosmic::iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                text: _,
                modifiers: _,
                ..
            }) => match key {
                Key::Named(Named::Escape) => Some(Msg::Cancel),
                _ => None,
            },
            _ => None,
        }));

        subscriptions.extend(self.surfaces.iter().map(|(id, surface)| match surface {
            Surface::PolkitDialog(state) => state.subscription().with(*id).map(Msg::PolkitDialog),
        }));
        if self.action_to_confirm.is_some() {
            subscriptions.push(time::every(Duration::from_millis(1000)).map(|_| Msg::Countdown));
        }

        iced::Subscription::batch(subscriptions)
    }

    fn view(&self) -> cosmic::prelude::Element<Self::Message> {
        unreachable!()
    }

    fn view_window(&self, id: SurfaceId) -> cosmic::Element<Msg> {
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
        } else if matches!(self.action_to_confirm, Some((c_id, _, _)) if c_id == id) {
            let cosmic_theme = self.core.system_theme().cosmic();
            let (_, power_action, countdown) = self.action_to_confirm.as_ref().unwrap();
            let action = match *power_action {
                OsdTask::EnterBios => "enter-bios",
                OsdTask::LogOut => "log-out",
                OsdTask::Restart => "restart",
                OsdTask::Shutdown => "shutdown",
            };

            let title = fl!(
                "confirm-title",
                HashMap::from_iter(vec![("action", action)])
            );
            let countdown = &countdown.to_string();
            let mut dialog = cosmic::widget::dialog()
                .title(title)
                .body(fl!(
                    "confirm-body",
                    HashMap::from_iter(vec![("action", action), ("countdown", countdown)])
                ))
                .primary_action(
                    button::custom(min_width_and_height(
                        text::body(fl!("confirm", HashMap::from_iter(vec![("action", action)])))
                            .into(),
                        142.0,
                        32.0,
                    ))
                    .padding([0, cosmic_theme.space_s()])
                    .id(CONFIRM_ID.clone())
                    .class(theme::Button::Suggested)
                    .on_press(Msg::Confirm),
                )
                .secondary_action(
                    button::custom(min_width_and_height(
                        text::body(fl!("cancel")).into(),
                        142.0,
                        32.0,
                    ))
                    .padding([0, cosmic_theme.space_s()])
                    .class(theme::Button::Standard)
                    .on_press(Msg::Cancel),
                )
                .icon(text_icon(
                    match power_action {
                        OsdTask::LogOut => "system-log-out-symbolic",
                        OsdTask::Restart | OsdTask::EnterBios => "system-restart-symbolic",
                        OsdTask::Shutdown => "system-shutdown-symbolic",
                    },
                    60,
                ));

            if matches!(power_action, OsdTask::Shutdown) {
                dialog = dialog.tertiary_action(
                    button::text(fl!("restart")).on_press(Msg::Action(OsdTask::Restart)),
                );
            }
            return Element::from(
                autosize(Element::from(container(dialog)), AUTOSIZE_DIALOG_ID.clone()).limits(
                    Limits::NONE
                        .min_width(1.)
                        .min_height(1.)
                        .max_width(900.)
                        .max_height(900.),
                ),
            );
        }
        iced::widget::text("").into() // XXX
    }

    fn dbus_activation(&mut self, msg: cosmic::dbus_activation::Message) -> Task<Msg> {
        match msg.msg {
            Details::Activate => {}
            Details::ActivateAction { action, .. } => {
                let Ok(cmd) = OsdTask::from_str(&action) else {
                    return Task::none();
                };
                if let Some(prev) = self.action_to_confirm.take() {
                    self.action_to_confirm = Some((prev.0, cmd, COUNTDOWN_LENGTH));
                } else {
                    let id = SurfaceId::unique();
                    self.action_to_confirm = Some((id, cmd, COUNTDOWN_LENGTH));
                    return get_layer_surface(SctkLayerSurfaceSettings {
                        id,
                        keyboard_interactivity: KeyboardInteractivity::Exclusive,
                        anchor: Anchor::empty(),
                        namespace: "dialog".into(),
                        size: None,
                        size_limits: Limits::NONE.min_width(1.0).min_height(1.0),
                        ..Default::default()
                    });
                }
            }
            Details::Open { .. } => {}
        }
        Task::none()
    }
}

pub fn main() -> iced::Result {
    let args = Args::parse();

    cosmic::app::run_single_instance::<App>(
        cosmic::app::Settings::default()
            .no_main_window(true)
            .exit_on_close(false),
        args,
    )
}

mod pipewire {
    // Copyright 2023 System76 <info@system76.com>
    // SPDX-License-Identifier: MPL-2.0

    use std::path::Path;
    use std::process::Stdio;
    use xdg::BaseDirectories;

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
        if let Ok(sounds_dirs) = xdg::BaseDirectories::with_prefix("sounds") {
            if let Some(path) =
                sounds_dirs.find_data_file("freedesktop/stereo/audio-volume-change.oga")
            {
                play(&path);
                return;
            }
        }
        log::error!("Sound file not found in XDG data directories");
    }
}

fn min_width_and_height(
    e: Element<Msg>,
    width: impl Into<Length>,
    height: impl Into<Length>,
) -> Column<Msg> {
    use iced::widget::{column, horizontal_space, row, vertical_space};
    column![
        row![e, vertical_space().height(height)].align_y(Alignment::Center),
        horizontal_space().width(width)
    ]
    .align_x(Alignment::Center)
}

fn text_icon(name: &str, size: u16) -> cosmic::widget::Icon {
    icon::from_name(name).size(size).symbolic(true).icon()
}
