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
        alignment::Horizontal,
        event::{
            self, listen_with,
            wayland::{self, LayerEvent, OutputEvent, OverlapNotifyEvent},
        },
        keyboard::{Key, key::Named},
        platform_specific::shell::commands::activation::request_token,
        time,
        window::Id as SurfaceId,
    },
    iced_runtime::platform_specific::wayland::layer_surface::{
        IcedOutput, SctkLayerSurfaceSettings,
    },
    iced_winit::commands::layer_surface::{
        Anchor, KeyboardInteractivity, destroy_layer_surface, get_layer_surface,
    },
    theme,
    widget::{Column, autosize::autosize, button, container, icon, row, text},
};
use cosmic_comp_config::input::TouchpadOverride;
use cosmic_settings_airplane_mode_subscription as airplane_mode;
use cosmic_settings_daemon_subscription as settings_daemon;
use cosmic_settings_sound_subscription::pulse;
use cosmic_settings_upower_subscription::kbdbacklight::{
    KeyboardBacklightUpdate, kbd_backlight_subscription,
};
use logind_zbus::manager::ManagerProxy;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Display,
    process::Stdio,
    str::FromStr,
    sync::LazyLock,
    time::{Duration, Instant},
};
use zbus::Connection;

// Type alias for Wayland output. Matches what's used in SctkLayerSurfaceSettings
type WlOutput = cosmic::cctk::sctk::reexports::client::protocol::wl_output::WlOutput;

const COUNTDOWN_LENGTH: u8 = 60;
static CONFIRM_ID: LazyLock<iced::id::Id> = LazyLock::new(|| iced::id::Id::new("confirm-id"));
static AUTOSIZE_DIALOG_ID: LazyLock<iced::id::Id> =
    LazyLock::new(|| iced::id::Id::new("autosize-id"));

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<OsdTask>,
}

#[derive(Debug, Serialize, Deserialize, Clone, clap::Subcommand)]
pub enum OsdTask {
    #[clap(about = "Display external display toggle indicator")]
    Display,
    #[clap(about = "Show numbers on all displays for identification")]
    IdentifyDisplays,
    #[clap(about = "Dismiss display identification numbers")]
    DismissDisplayIdentifiers,
    #[clap(about = "Toggle the on screen display and start the log out timer")]
    LogOut,
    #[clap(about = "Toggle the on screen display and start the restart timer")]
    Restart,
    #[clap(about = "Toggle the on screen display and start the shutdown timer")]
    Shutdown,
    #[clap(about = "Display touchpad toggle indicator")]
    Touchpad,
    #[clap(about = "Toggle the on screen display and start the restart to bios timer")]
    EnterBios,
    ConfirmHeadphones {
        #[arg(long)]
        card_name: String,
        #[arg(long)]
        headphone_profile: String,
        #[arg(long)]
        headset_profile: String,
        #[arg(long)]
        headset_port_name: String,
        #[clap(skip)]
        selected_headset: bool,
    },
}

impl OsdTask {
    fn perform(self) -> Task<Msg> {
        let msg = |m| cosmic::action::app(Msg::Zbus(m));
        match self {
            OsdTask::EnterBios => cosmic::task::future(restart(true)).map(msg),
            OsdTask::LogOut => cosmic::task::future(log_out()).map(msg),
            OsdTask::Restart => cosmic::task::future(restart(false)).map(msg),
            OsdTask::Shutdown => cosmic::task::future(shutdown()).map(msg),
            OsdTask::ConfirmHeadphones {
                card_name,
                headphone_profile,
                headset_profile,
                headset_port_name,
                selected_headset,
            } => cosmic::task::future(confirm_headphones(
                card_name,
                headphone_profile,
                headset_profile,
                headset_port_name,
                selected_headset,
            ))
            .map(msg),
            OsdTask::Touchpad => Task::none(),
            OsdTask::Display => Task::none(),
            OsdTask::IdentifyDisplays => Task::none(),
            OsdTask::DismissDisplayIdentifiers => Task::none(),
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

async fn confirm_headphones(
    card_name: String,
    headphone_profile: String,
    headset_profile: String,
    headset_port_name: String,
    selected_headset: bool,
) -> zbus::Result<()> {
    use tokio::process::Command;

    let profile = if selected_headset {
        headset_profile
    } else {
        headphone_profile
    };

    let status = Command::new("pactl")
        .arg("set-card-profile")
        .arg(card_name)
        .arg(profile)
        .status()
        .await;

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            return Err(zbus::Error::Failure(format!(
                "pactl exited with status: {}",
                s
            )));
        }

        Err(e) => {
            return Err(zbus::Error::Failure(format!("Failed to run pactl: {}", e)));
        }
    };

    if selected_headset {
        let output = Command::new("pactl")
            .arg("get-default-source")
            .stdout(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(zbus::Error::Failure(
                "Failed to get source name.".to_string(),
            ));
        }

        let source_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

        let status = Command::new("pactl")
            .arg("set-source-port")
            .arg(&source_name)
            .arg(&headset_port_name)
            .status()
            .await?;

        if status.success() {
            Ok(())
        } else {
            Err(zbus::Error::Failure("Failed to set port.".to_string()))
        }
    } else {
        Ok(())
    }
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

#[derive(Clone, Copy, Debug)]
pub enum DisplayMode {
    All,
    External,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Action(OsdTask),
    Confirm,
    Cancel,
    Countdown,
    DBus(dbus::Event),
    Display(Option<DisplayMode>),
    Headphones(bool),
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
    SoundSettings,
    TouchpadEnabled(Option<TouchpadOverride>),
    ActivationToken(Option<String>),
    DisplayIdentifierSurface((SurfaceId, osd_indicator::Msg)),
    ResetDisplayIdentifierTimer(SurfaceId),
    CreateDisplayIdentifiers(Vec<(String, u32)>),
    DismissDisplayIdentifiers,
    OutputInfo(WlOutput, String),
    OutputRemoved(WlOutput),
}

enum Surface {
    PolkitDialog(polkit_dialog::State),
    OsdIndicator(osd_indicator::State),
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
    wayland_outputs: HashMap<String, (WlOutput, String)>,
    display_identifier_displays: HashMap<SurfaceId, String>,
    identifiers_dismissed: bool,
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

    fn trigger_identify_displays(&self) -> cosmic::app::Task<Msg> {
        cosmic::task::future(async move {
            // Add a small delay to allow cosmic-randr to sync with display changes
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            let Ok(output_lists) = cosmic_randr_shell::list().await else {
                log::error!("Failed to list displays with cosmic-randr");
                return Msg::CreateDisplayIdentifiers(Vec::new());
            };

            // Get all enabled outputs and number them the same way cosmic-settings does:
            // Sort alphabetically by output name, then assign numbers 1, 2, 3...
            use std::collections::BTreeMap;

            let sorted_outputs: BTreeMap<&str, _> = output_lists
                .outputs
                .iter()
                .filter(|(_, o)| o.enabled)
                .map(|(key, output)| (output.name.as_str(), (key, output)))
                .collect();

            let displays: Vec<(String, u32)> = sorted_outputs
                .into_iter()
                .enumerate()
                .map(|(index, (name, _))| (name.to_string(), (index + 1) as u32))
                .collect();

            log::debug!(
                "Identified {} enabled displays: {:?}",
                displays.len(),
                displays
            );

            // Only show identifiers if there are 2 or more displays
            if displays.len() < 2 {
                log::info!(
                    "Skipping display identifiers: only {} enabled display(s)",
                    displays.len()
                );
                return Msg::CreateDisplayIdentifiers(Vec::new());
            }

            Msg::CreateDisplayIdentifiers(displays)
        })
        .map(cosmic::Action::App)
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
                wayland_outputs: HashMap::new(),
                display_identifier_displays: HashMap::new(),
                identifiers_dismissed: false,
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
                // Some actions don't require confirmation and execute immediately
                if matches!(action, OsdTask::IdentifyDisplays) {
                    // Clear dismissed flag to allow showing identifiers
                    self.identifiers_dismissed = false;
                    return self.trigger_identify_displays();
                } else if matches!(action, OsdTask::DismissDisplayIdentifiers) {
                    return Task::done(cosmic::Action::App(Msg::DismissDisplayIdentifiers));
                } else if matches!(action, OsdTask::Restart)
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
                        let a = a.clone();

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
            Msg::DisplayIdentifierSurface((id, msg)) => {
                if let Some(Surface::OsdIndicator(state)) = self.surfaces.remove(&id) {
                    let (state, cmd) = state.update(msg);
                    if let Some(state) = state {
                        self.surfaces.insert(id, Surface::OsdIndicator(state));
                    } else {
                        self.display_identifier_displays.remove(&id);
                        log::debug!("Display identifier surface {:?} closed", id);
                    }
                    return cmd.map(move |msg| {
                        cosmic::action::app(Msg::DisplayIdentifierSurface((id, msg)))
                    });
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
                    if let Some(max) = self.max_display_brightness {
                        if max <= 20 {
                            // Coarse displays: rung_ratio=(raw+1)/20
                            let rung_ratio = ((brightness + 1) as f64) / 20.0;
                            self.create_indicator(osd_indicator::Params::DisplayBrightness(rung_ratio))
                        } else {
                            // Fine displays: exact integer percent from raw/max
                            let ratio = (brightness as f64) / (max as f64);
                            self.create_indicator(osd_indicator::Params::DisplayBrightnessExact(ratio))
                        }
                    } else {
                        Task::none()
                    }
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
                    log::error!("D-Bus error: {}", e);
                }
                Task::none()
            }

            Msg::ActivationToken(token) => {
                let mut envs = Vec::new();
                if let Some(token) = token {
                    envs.push(("XDG_ACTIVATION_TOKEN".to_string(), token.clone()));
                    envs.push(("DESKTOP_STARTUP_ID".to_string(), token));
                }
                Task::perform(
                    cosmic::desktop::spawn_desktop_exec("cosmic-settings sound", envs, None, false),
                    |()| cosmic::action::app(Msg::Cancel),
                )
            }
            Msg::SoundSettings => {
                if let Some(id) = self.action_to_confirm.as_ref().map(|a| a.0) {
                    request_token(Some(String::from(Self::APP_ID)), Some(id))
                        .map(move |token| cosmic::Action::App(Msg::ActivationToken(token)))
                } else {
                    log::error!("Failed ot spawn cosmic-settings.");
                    Task::none()
                }
            }
            Msg::Headphones(value) => {
                if let Some((
                    _,
                    OsdTask::ConfirmHeadphones {
                        selected_headset, ..
                    },
                    _,
                )) = self.action_to_confirm.as_mut()
                {
                    *selected_headset = value;
                }
                Task::none()
            }
            Msg::TouchpadEnabled(enabled) => {
                let Some(enabled) = enabled else {
                    log::warn!("TouchpadEnabled event received with None value");
                    return Task::none();
                };
                // Show the OSD indicator for touchpad enabled/disabled
                let id = SurfaceId::unique();
                let (state, cmd) =
                    osd_indicator::State::new(id, osd_indicator::Params::TouchpadEnabled(enabled));
                self.indicator = Some((id, state));
                cmd.map(|x| cosmic::Action::App(Msg::OsdIndicator(x)))
            }
            Msg::Display(enabled) => {
                let Some(enabled) = enabled else {
                    log::warn!("Display event received with None value");
                    return Task::none();
                };
                let id = SurfaceId::unique();
                let (state, cmd) =
                    osd_indicator::State::new(id, osd_indicator::Params::DisplayToggle(enabled));
                self.indicator = Some((id, state));
                cmd.map(|x| cosmic::Action::App(Msg::OsdIndicator(x)))
            }
            Msg::OutputInfo(output, name) => {
                let is_new = !self.wayland_outputs.contains_key(&name);
                self.wayland_outputs
                    .insert(name.clone(), (output, name.clone()));

                if is_new {
                    log::debug!("Display '{}' added to wayland outputs tracking", name);
                }
                Task::none()
            }
            Msg::OutputRemoved(output) => {
                // Find and remove the output from our tracking map
                let mut removed_name = None;
                self.wayland_outputs.retain(|name, (out, _)| {
                    if out == &output {
                        removed_name = Some(name.clone());
                        false
                    } else {
                        true
                    }
                });

                if let Some(name) = removed_name {
                    log::info!(
                        "Display '{}' disconnected, updating display identifiers",
                        name
                    );
                    // Trigger display identifier OSD to show the updated numbering
                    Task::done(cosmic::Action::App(Msg::Action(OsdTask::IdentifyDisplays)))
                } else {
                    log::warn!(
                        "OutputRemoved event received but display not found in wayland_outputs"
                    );
                    Task::none()
                }
            }
            Msg::CreateDisplayIdentifiers(displays) => {
                if displays.is_empty() {
                    log::warn!("CreateDisplayIdentifiers called with empty display list");
                    return Task::none();
                }

                if self.identifiers_dismissed {
                    log::debug!(
                        "Ignoring CreateDisplayIdentifiers: identifiers were explicitly dismissed"
                    );
                    return Task::none();
                }

                log::info!(
                    "Creating display identifiers for {} displays: {:?}",
                    displays.len(),
                    displays
                );
                log::debug!(
                    "Current wayland_outputs: {:?}",
                    self.wayland_outputs.keys().collect::<Vec<_>>()
                );

                self.identifiers_dismissed = false;

                let mut tasks = Vec::new();

                let requested_displays: HashMap<String, u32> = displays.iter().cloned().collect();

                let mut existing_identifiers: HashMap<String, (SurfaceId, u32)> = HashMap::new();
                for (id, display_name) in &self.display_identifier_displays {
                    if let Some(Surface::OsdIndicator(state)) = self.surfaces.get(id) {
                        if let osd_indicator::Params::DisplayNumber(num) = state.params() {
                            existing_identifiers.insert(display_name.clone(), (*id, *num));
                        }
                    }
                }

                log::debug!("Found {} existing identifiers", existing_identifiers.len());

                let mut kept_ids = std::collections::HashSet::new();

                // Process each requested display
                for (display_name, display_number) in &displays {
                    if let Some((existing_id, existing_number)) =
                        existing_identifiers.get(display_name)
                    {
                        // We have an existing identifier for this display
                        if existing_number == display_number {
                            log::debug!(
                                "Display '{}' already has correct identifier (number {}), resetting timer",
                                display_name,
                                display_number
                            );
                            kept_ids.insert(*existing_id);
                            tasks.push(Task::done(cosmic::Action::App(
                                Msg::ResetDisplayIdentifierTimer(*existing_id),
                            )));
                        } else {
                            log::debug!(
                                "Display '{}' has wrong number (has {}, needs {}), recreating",
                                display_name,
                                existing_number,
                                display_number
                            );
                            self.surfaces.remove(existing_id);
                            self.display_identifier_displays.remove(existing_id);
                            tasks.push(destroy_layer_surface(*existing_id));

                            let id = SurfaceId::unique();
                            log::debug!(
                                "Creating identifier surface for display '{}' (number {})",
                                display_name,
                                display_number
                            );

                            let iced_output =
                                if let Some((output, _)) = self.wayland_outputs.get(display_name) {
                                    IcedOutput::Output(output.clone())
                                } else {
                                    log::warn!(
                                        "Display '{}' not found in wayland_outputs",
                                        display_name
                                    );
                                    IcedOutput::Active
                                };

                            let (state, cmd) = osd_indicator::State::new_with_output(
                                id,
                                osd_indicator::Params::DisplayNumber(*display_number),
                                iced_output,
                            );

                            self.surfaces.insert(id, Surface::OsdIndicator(state));
                            self.display_identifier_displays
                                .insert(id, display_name.clone());
                            tasks.push(cmd.map(move |msg| {
                                cosmic::action::app(Msg::DisplayIdentifierSurface((id, msg)))
                            }));
                        }
                    } else {
                        // No existing identifier for this display, create new one
                        let id = SurfaceId::unique();
                        log::debug!(
                            "Creating identifier surface for display '{}' (number {})",
                            display_name,
                            display_number
                        );

                        let iced_output = if let Some((output, _)) =
                            self.wayland_outputs.get(display_name)
                        {
                            IcedOutput::Output(output.clone())
                        } else {
                            log::warn!("Display '{}' not found in wayland_outputs", display_name);
                            IcedOutput::Active
                        };

                        let (state, cmd) = osd_indicator::State::new_with_output(
                            id,
                            osd_indicator::Params::DisplayNumber(*display_number),
                            iced_output,
                        );

                        self.surfaces.insert(id, Surface::OsdIndicator(state));
                        self.display_identifier_displays
                            .insert(id, display_name.clone());
                        tasks.push(cmd.map(move |msg| {
                            cosmic::action::app(Msg::DisplayIdentifierSurface((id, msg)))
                        }));
                    }
                }

                // Remove any identifiers that weren't in the requested list
                let ids_to_remove: Vec<SurfaceId> = existing_identifiers
                    .iter()
                    .filter_map(|(name, (id, _))| {
                        if !requested_displays.contains_key(name) && !kept_ids.contains(id) {
                            Some(*id)
                        } else {
                            None
                        }
                    })
                    .collect();

                if !ids_to_remove.is_empty() {
                    log::debug!("Removing {} obsolete identifiers", ids_to_remove.len());
                    for id in ids_to_remove {
                        self.surfaces.remove(&id);
                        self.display_identifier_displays.remove(&id);
                        tasks.push(destroy_layer_surface(id));
                    }
                }

                Task::batch(tasks)
            }
            Msg::ResetDisplayIdentifierTimer(id) => {
                if let Some(Surface::OsdIndicator(state)) = self.surfaces.get_mut(&id) {
                    return state.reset_display_identifier_timer().map(move |msg| {
                        cosmic::action::app(Msg::DisplayIdentifierSurface((id, msg)))
                    });
                }
                Task::none()
            }
            Msg::DismissDisplayIdentifiers => {
                let ids_to_remove: Vec<SurfaceId> = self
                    .surfaces
                    .iter()
                    .filter_map(|(id, surface)| {
                        if let Surface::OsdIndicator(state) = surface {
                            if matches!(state.params(), osd_indicator::Params::DisplayNumber(_)) {
                                Some(*id)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                log::info!(
                    "Dismissing {} display identifier surfaces",
                    ids_to_remove.len()
                );

                // Mark as explicitly dismissed to prevent race conditions
                self.identifiers_dismissed = true;

                let mut tasks = Vec::new();
                for id in ids_to_remove {
                    self.surfaces.remove(&id);
                    self.display_identifier_displays.remove(&id);
                    tasks.push(destroy_layer_surface(id));
                }

                Task::batch(tasks)
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
                    event::wayland::Event::OverlapNotify(event, ..) => Some(Msg::Overlap(event)),
                    wayland::Event::Layer(LayerEvent::Unfocused, ..) => Some(Msg::Cancel),
                    event::wayland::Event::Output(output_event, output) => {
                        match output_event {
                            OutputEvent::Created(Some(info)) => {
                                // Track this output for creating per-display surfaces
                                if let Some(name) = info.name.clone() {
                                    log::debug!("Output Created: {}", name);
                                    Some(Msg::OutputInfo(output.clone(), name))
                                } else {
                                    None
                                }
                            }
                            OutputEvent::InfoUpdate(info) => {
                                // Update existing output info
                                if let Some(name) = info.name.clone() {
                                    log::debug!("Output InfoUpdate: {}", name);
                                    Some(Msg::OutputInfo(output.clone(), name))
                                } else {
                                    None
                                }
                            }
                            OutputEvent::Removed => {
                                log::debug!("Output Removed");
                                Some(Msg::OutputRemoved(output.clone()))
                            }
                            OutputEvent::Created(None) => None,
                        }
                    }
                    _ => None,
                }
            }
            cosmic::iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: Key::Named(Named::Escape),
                text: _,
                modifiers: _,
                ..
            }) => Some(Msg::Cancel),
            _ => None,
        }));

        subscriptions.extend(
            self.surfaces
                .iter()
                .filter_map(|(id, surface)| match surface {
                    Surface::PolkitDialog(state) => {
                        Some(state.subscription().with(*id).map(Msg::PolkitDialog))
                    }
                    Surface::OsdIndicator(_) => None, // OSD indicators don't have subscriptions
                }),
        );
        if self.action_to_confirm.is_some() {
            subscriptions.push(time::every(Duration::from_millis(1000)).map(|_| Msg::Countdown));
        }

        iced::Subscription::batch(subscriptions)
    }

    fn view(&self) -> cosmic::prelude::Element<'_, Self::Message> {
        unreachable!()
    }

    fn view_window(&self, id: SurfaceId) -> cosmic::Element<'_, Msg> {
        if let Some(surface) = self.surfaces.get(&id) {
            return match surface {
                Surface::PolkitDialog(state) => {
                    state.view().map(move |msg| Msg::PolkitDialog((id, msg)))
                }
                Surface::OsdIndicator(state) => state
                    .view()
                    .map(move |msg| Msg::DisplayIdentifierSurface((id, msg))),
            };
        } else if let Some((indicator_id, state)) = &self.indicator {
            if id == *indicator_id {
                return state.view().map(Msg::OsdIndicator);
            }
        } else if matches!(self.action_to_confirm, Some((c_id, _, _)) if c_id == id) {
            let cosmic_theme = self.core.system_theme().cosmic();
            let (_, cur_action, countdown) = self.action_to_confirm.as_ref().unwrap();
            let action = match *cur_action {
                OsdTask::EnterBios => "enter-bios",
                OsdTask::LogOut => "log-out",
                OsdTask::Restart => "restart",
                OsdTask::Shutdown => "shutdown",
                OsdTask::ConfirmHeadphones { .. } => "confirm-device-type",
                OsdTask::Touchpad => "touchpad",
                OsdTask::Display => "external-display",
                OsdTask::IdentifyDisplays => "identify-displays",
                OsdTask::DismissDisplayIdentifiers => "dismiss-display-identifiers",
            };

            let title = fl!(
                "confirm-title",
                HashMap::from_iter(vec![("action", action)])
            );
            let countdown = &countdown.to_string();
            let mut dialog = cosmic::widget::dialog().title(title);

            dialog = dialog
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
                );
            let t = self.core.system_theme().cosmic();
            dialog = if matches!(cur_action, OsdTask::ConfirmHeadphones { .. }) {
                dialog
                    .tertiary_action(
                        button::text(fl!("sound-settings")).on_press(Msg::SoundSettings),
                    )
                    .control(
                        container(
                            row()
                                .push(
                                    cosmic::widget::column()
                                        .push(
                                            cosmic::widget::button::custom_image_button(
                                                container(
                                                    cosmic::widget::icon::from_name(
                                                        "audio-headphones-symbolic",
                                                    )
                                                    .size(64)
                                                    .icon(),
                                                )
                                                .padding(t.space_m()),
                                                None,
                                            )
                                            .class(cosmic::theme::style::Button::Image)
                                            .selected(matches!(
                                                self.action_to_confirm,
                                                Some((_, OsdTask::ConfirmHeadphones {
                                                    selected_headset,
                                                    ..
                                                },_)) if !selected_headset
                                            ))
                                            .on_press(Msg::Headphones(false)),
                                        )
                                        .push(text(fl!("headphones")))
                                        .align_x(Alignment::Center)
                                        .spacing(t.space_xxxs()),
                                )
                                .push(
                                    cosmic::widget::column()
                                        .push(
                                            cosmic::widget::button::custom_image_button(
                                                container(
                                                    cosmic::widget::icon::from_name(
                                                        "audio-headset-symbolic",
                                                    )
                                                    .size(64)
                                                    .icon(),
                                                )
                                                .padding(t.space_m()),
                                                None,
                                            )
                                            .class(cosmic::theme::style::Button::Image)
                                            .selected(matches!(
                                                self.action_to_confirm,
                                                Some((_, OsdTask::ConfirmHeadphones {
                                                    selected_headset,
                                                    ..
                                                },_)) if selected_headset
                                            ))
                                            .on_press(Msg::Headphones(true)),
                                        )
                                        .push(text(fl!("headset")))
                                        .align_x(Alignment::Center)
                                        .spacing(t.space_xxxs()),
                                )
                                .spacing(t.space_l()),
                        )
                        .width(Length::Fixed(522.))
                        .align_x(Horizontal::Center),
                    )
            } else {
                dialog
                    .icon(text_icon(
                        match cur_action {
                            OsdTask::LogOut => "system-log-out-symbolic",
                            OsdTask::Restart | OsdTask::EnterBios => "system-restart-symbolic",
                            OsdTask::Shutdown => "system-shutdown-symbolic",
                            _ => unreachable!(),
                        },
                        60,
                    ))
                    .body(fl!(
                        "confirm-body",
                        HashMap::from_iter(vec![("action", action), ("countdown", countdown)])
                    ))
            };

            if matches!(cur_action, OsdTask::Shutdown) {
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
                if let OsdTask::Touchpad = cmd {
                    return cosmic::task::future(async move {
                        use cosmic_config::{ConfigGet, ConfigSet};

                        let (tx, rx) = tokio::sync::oneshot::channel();

                        std::thread::spawn({
                            move || {
                                if let Ok(helper) =
                                    cosmic_config::Config::new("com.system76.CosmicComp", 1)
                                {
                                    let mut enabled = helper
                                        .get::<TouchpadOverride>("input_touchpad_override")
                                        .unwrap_or_default();
                                    if matches!(enabled, TouchpadOverride::None) {
                                        // If it's on auto, we consider it enabled
                                        enabled = TouchpadOverride::ForceDisable;
                                    } else {
                                        enabled = TouchpadOverride::None;
                                    }
                                    if let Err(err) = helper.set("input_touchpad_override", enabled)
                                    {
                                        log::error!("Failed to set touchpad override: {}", err);
                                        return;
                                    }
                                    let _ = tx.send(enabled);
                                } else {
                                    log::error!("Failed to load CosmicComp config for touchpad");
                                }
                            }
                        });
                        Msg::TouchpadEnabled(rx.await.ok())
                    })
                    .map(cosmic::Action::App);
                } else if let OsdTask::Display = cmd {
                    return cosmic::task::future(async move {
                        let enabled;

                        let Ok(mut output_lists) = cosmic_randr_shell::list().await else {
                            log::error!("Failed to list displays with cosmic-randr");
                            return Msg::Display(None);
                        };

                        let mut enabled_positions = output_lists
                            .outputs
                            .values()
                            .filter(|o| o.enabled)
                            .filter_map(|o| {
                                o.current.and_then(|c_mode| {
                                    output_lists.modes.get(c_mode).map(|m| (o.position, m.size))
                                })
                            })
                            .collect::<Vec<_>>();
                        enabled_positions.sort_by_key(|p| p.0.0);

                        let other_enabled = output_lists.outputs.values().any(|o| {
                            !(o.name.starts_with("eDP-")
                                || o.name.starts_with("LVDS-")
                                || o.name.starts_with("DSI-"))
                                && o.enabled
                        });

                        let mut internal = output_lists
                            .outputs
                            .values_mut()
                            .filter(|o| {
                                o.name.starts_with("eDP-")
                                    || o.name.starts_with("LVDS-")
                                    || o.name.starts_with("DSI-")
                            })
                            .collect::<Vec<_>>();
                        if internal.is_empty() {
                            log::error!("No internal display found");
                            return Msg::Display(None);
                        }
                        let all_internal_enabled = internal.iter().all(|o| o.enabled);

                        if all_internal_enabled {
                            if other_enabled {
                                enabled = Some(DisplayMode::External);
                            } else {
                                log::info!("Not disabling the only enabled display");
                                return Msg::Display(None);
                            }
                            for o in internal.iter_mut() {
                                o.enabled = false;
                            }
                        } else {
                            enabled = Some(DisplayMode::All);
                            for o in internal.iter_mut() {
                                o.enabled = true;
                                let Some(mut mode) = o.modes.first().copied() else {
                                    continue;
                                };
                                for m in &o.modes {
                                    let Some(v) = output_lists.modes.get(*m) else {
                                        continue;
                                    };
                                    if v.preferred {
                                        mode = *m;
                                        break;
                                    }
                                }
                                o.current = Some(mode);
                                // try to place position to the right of the leftmost enabled display, and iterate to the right
                                // must check if position/size overlaps with any other enabled display

                                for p in enabled_positions.iter() {
                                    let position = (p.0.0 + p.1.0 as i32, p.0.1);
                                    let Some(v) = output_lists.modes.get(mode) else {
                                        continue;
                                    };
                                    let size = v.size;
                                    let overlaps = enabled_positions.iter().any(|p2| {
                                        !(position.0 >= p2.0.0 + p2.1.0 as i32
                                            || position.0 + size.0 as i32 <= p2.0.0
                                            || position.1 >= p2.0.1 + p2.1.1 as i32
                                            || position.1 + size.1 as i32 <= p2.0.1)
                                    });

                                    if !overlaps {
                                        o.position = position;
                                        enabled_positions.push((position, size));
                                        enabled_positions.sort_by_key(|p| p.0.0);
                                        break;
                                    }
                                }
                            }
                        }

                        let mut task = tokio::process::Command::new("cosmic-randr");
                        task.arg("kdl");

                        task.stdin(Stdio::piped());
                        let Ok(mut p) = task.spawn() else {
                            return Msg::Display(None);
                        };

                        let kdl_doc = kdl::KdlDocument::from(output_lists).to_string();
                        use tokio::io::AsyncWriteExt;

                        if let Some(mut stdin) = p.stdin.take() {
                            if let Err(err) = stdin.write_all(kdl_doc.as_bytes()).await {
                                log::error!("Failed to write KDL to stdin: {err:?}");
                            }
                            if let Err(err) = stdin.flush().await {
                                log::error!("Failed to flush stdin: {err:?}");
                            }
                        }

                        log::debug!("executing {task:?}");
                        let status = p.wait().await;
                        if let Err(err) = status {
                            log::error!("Randr error: {err:?}");
                        };

                        Msg::Display(enabled)
                    });
                } else if let OsdTask::IdentifyDisplays = cmd {
                    // Clear dismissed flag to allow showing identifiers
                    self.identifiers_dismissed = false;
                    return self.trigger_identify_displays();
                } else if let OsdTask::DismissDisplayIdentifiers = cmd {
                    return Task::done(cosmic::Action::App(Msg::DismissDisplayIdentifiers));
                }

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

    /// Plays an audio file.
    pub fn play(path: &Path) {
        let _result = tokio::process::Command::new("pw-play")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("--media-role")
            .arg("Notification")
            .arg(path)
            .spawn();
    }

    pub fn play_audio_volume_change() {
        let sounds_dirs = xdg::BaseDirectories::with_prefix("sounds");
        if let Some(path) = sounds_dirs.find_data_file("freedesktop/stereo/audio-volume-change.oga")
        {
            play(&path);
            return;
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
