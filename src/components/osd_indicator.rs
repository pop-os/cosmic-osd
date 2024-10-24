// TODO: animation to fade in/out?
// TODO: Dismiss on click?

use cosmic::{
    iced::{self, window::Id as SurfaceId, Alignment, Border, Length},
    iced_runtime::platform_specific::wayland::layer_surface::{
        IcedMargin, SctkLayerSurfaceSettings,
    },
    iced_winit::commands::layer_surface::{
        destroy_layer_surface, get_layer_surface, Anchor, KeyboardInteractivity, Layer,
    },
    widget, Element, Task,
};
use futures::future::{abortable, AbortHandle, Aborted};
use once_cell::sync::Lazy;
use std::time::Duration;

pub static OSD_INDICATOR_ID: Lazy<widget::Id> =
    Lazy::new(|| widget::Id::new("osd-indicator".to_string()));

#[derive(Debug)]
pub enum Params {
    DisplayBrightness(f64),
    KeyboardBrightness(f64),
    SinkVolume(u32, bool),
    SourceVolume(u32, bool),
    AirplaneMode(bool),
}

impl Params {
    fn icon_name(&self) -> &'static str {
        match self {
            Self::DisplayBrightness(_) => "display-brightness-symbolic",
            Self::KeyboardBrightness(_) => "keyboard-brightness-symbolic",
            Self::AirplaneMode(true) => "airplane-mode-symbolic",
            Self::AirplaneMode(false) => "airplane-mode-disabled-symbolic",
            Self::SinkVolume(volume, muted) => {
                if *volume == 0 || *muted {
                    "audio-volume-muted-symbolic"
                } else if *volume < 33 {
                    "audio-volume-low-symbolic"
                } else if *volume < 66 {
                    "audio-volume-medium-symbolic"
                } else if *volume <= 100 {
                    "audio-volume-high-symbolic"
                } else {
                    "audio-volume-overamplified-symbolic"
                }
            }
            Self::SourceVolume(volume, muted) => {
                if *volume == 0 || *muted {
                    "microphone-sensitivity-muted-symbolic"
                } else if *volume < 33 {
                    "microphone-sensitivity-low-symbolic"
                } else if *volume < 66 {
                    "microphone-sensitivity-medium-symbolic"
                } else {
                    "microphone-sensitivity-high-symbolic"
                }
            }
        }
    }

    fn value(&self) -> Option<u32> {
        match self {
            Self::DisplayBrightness(value) => Some((*value * 100.) as u32),
            Self::KeyboardBrightness(value) => Some((*value * 100.) as u32),
            Self::SinkVolume(_, true) => Some(0),
            Self::SourceVolume(_, true) => Some(0),
            Self::SinkVolume(value, false) => Some(*value),
            Self::SourceVolume(value, false) => Some(*value),
            Self::AirplaneMode(_) => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    Ignore,
    Close,
}

#[derive(Debug)]
pub struct State {
    id: SurfaceId,
    params: Params,
    timer_abort: AbortHandle,
}

fn close_timer() -> (Task<Msg>, AbortHandle) {
    let (future, timer_abort) = abortable(async {
        let duration = Duration::from_secs(3);
        tokio::time::sleep(duration).await;
    });
    let command = Task::perform(future, |res| {
        if res == Err(Aborted) {
            Msg::Ignore
        } else {
            Msg::Close
        }
    });
    (command, timer_abort)
}

impl State {
    pub fn new(id: SurfaceId, params: Params) -> (Self, Task<Msg>) {
        // Anchor to bottom right, with margin?
        let mut cmds = vec![];
        cmds.push(get_layer_surface(SctkLayerSurfaceSettings {
            id,
            pointer_interactivity: false,
            keyboard_interactivity: KeyboardInteractivity::None,
            namespace: "osd".into(),
            layer: Layer::Overlay,
            size: None,
            anchor: Anchor::BOTTOM,
            margin: IcedMargin {
                bottom: 48,
                ..Default::default()
            },
            ..Default::default()
        }));
        let (cmd, timer_abort) = close_timer();
        cmds.push(cmd);
        (
            Self {
                id,
                params,
                timer_abort,
            },
            Task::batch(cmds),
        )
    }

    // Re-use OSD surface to show a different OSD
    // Resets close timer
    pub fn replace_params(&mut self, params: Params) -> Task<Msg> {
        self.params = params;
        // Reset timer
        self.timer_abort.abort();
        let (cmd, timer_abort) = close_timer();
        self.timer_abort = timer_abort;
        cmd
    }

    pub fn view(&self) -> Element<'_, Msg> {
        let icon = widget::icon::from_name(self.params.icon_name());

        // Use large radius on value-OSD to enforce pill-shape with "Round" system style
        let radius;

        let osd_contents = if let Some(value) = self.params.value() {
            radius = cosmic::theme::active().cosmic().radius_l();
            widget::container(
                iced::widget::row![
                    widget::container(icon.size(20))
                        .width(Length::Fixed(32.0))
                        .align_x(Alignment::Center),
                    widget::text::body(format!("{}%", value))
                        .width(Length::Fixed(32.0))
                        .align_x(Alignment::Center),
                    widget::horizontal_space().width(Length::Fixed(8.0)),
                    widget::progress_bar(0. ..=100., value as f32)
                        .height(4)
                        .width(Length::Fixed(266.0)),
                ]
                .align_y(Alignment::Center),
            )
            .width(Length::Fixed(392.0))
            .height(Length::Fixed(52.0))
        } else {
            radius = cosmic::theme::active().cosmic().radius_m();
            const ICON_SIZE: u16 = 112;
            widget::container(icon.size(ICON_SIZE))
                .width(ICON_SIZE + 2 * cosmic::theme::active().cosmic().space_l())
                .height(ICON_SIZE + 2 * cosmic::theme::active().cosmic().space_s())
        };

        // Define overall style of OSD container
        let container_style =
            cosmic::theme::Container::custom(move |theme| widget::container::Style {
                text_color: Some(theme.cosmic().on_bg_color().into()),
                background: Some(iced::Color::from(theme.cosmic().bg_color()).into()),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: theme.cosmic().bg_divider().into(),
                },
                shadow: Default::default(),
                icon_color: Some(theme.cosmic().on_bg_color().into()),
            });

        widget::autosize::autosize(
            osd_contents
                .class(container_style)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            OSD_INDICATOR_ID.clone(),
        )
        .min_width(1.)
        .min_height(1.)
        .into()
    }

    pub fn update(self, msg: Msg) -> (Option<Self>, Task<Msg>) {
        log::trace!("indicator msg: {:?}", msg);
        match msg {
            Msg::Ignore => (Some(self), Task::none()),
            Msg::Close => (None, destroy_layer_surface(self.id)),
        }
    }
}
