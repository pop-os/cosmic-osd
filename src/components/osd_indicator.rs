// TODO: animation to fade in/out?
// TODO: Dismiss on click?

use crate::{components::app::DisplayMode, config};
use cosmic::{
    Apply, Element, Task,
    iced::{self, Alignment, Border, Length, window::Id as SurfaceId},
    iced_runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
    iced_winit::commands::{
        layer_surface::{
            Anchor, KeyboardInteractivity, Layer, destroy_layer_surface, get_layer_surface,
        },
        overlap_notify::overlap_notify,
    },
    widget::{self, horizontal_space, vertical_space},
};
use cosmic_comp_config::input::TouchpadOverride;
use futures::future::{AbortHandle, Aborted, abortable};
use std::sync::LazyLock;
use std::time::Duration;

pub static OSD_INDICATOR_ID: LazyLock<widget::Id> =
    LazyLock::new(|| widget::Id::new("osd-indicator".to_string()));

#[derive(Debug)]
pub enum Params {
    DisplayBrightness(f64),
    DisplayToggle(DisplayMode),
    KeyboardBrightness(f64),
    SinkVolume(u32, bool),
    SourceVolume(u32, bool),
    AirplaneMode(bool),
    TouchpadEnabled(TouchpadOverride),
}

impl Params {
    fn icon_name(&self) -> &'static str {
        match self {
            Self::DisplayBrightness(_) => "display-brightness-symbolic",
            Self::DisplayToggle(DisplayMode::All) => "laptop-symbolic",
            Self::DisplayToggle(DisplayMode::External) => "display-symbolic",
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
            Self::TouchpadEnabled(TouchpadOverride::None) => "input-touchpad-symbolic",
            Self::TouchpadEnabled(TouchpadOverride::ForceDisable) => "touchpad-disabled-symbolic",
        }
    }

    fn value(&self) -> Option<u32> {
        match self {
            Self::DisplayBrightness(value) => {
                // Round to nearest percent, and ensure non-zero values never display as 0%.
                // Prevents OSD from showing "0%" when brightness is clamped > 0 by the daemon.
                let pct = (*value * 100.).round();
                Some(if pct == 0.0 && *value > 0.0 {
                    1
                } else {
                    pct as u32
                })
            }
            Self::KeyboardBrightness(value) => Some((*value * 100.) as u32),
            Self::SinkVolume(_, true) => Some(0),
            Self::SourceVolume(_, true) => Some(0),
            Self::SinkVolume(value, false) => Some(*value),
            Self::SourceVolume(value, false) => Some(*value),
            Self::AirplaneMode(_) => None,
            Self::TouchpadEnabled(_) => None,
            Self::DisplayToggle(_) => None,
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
    pub margin: (i32, i32, i32, i32),
    amplification_sink: bool,
    amplification_source: bool,
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
            ..Default::default()
        }));
        cmds.push(overlap_notify(id, true));
        let (cmd, timer_abort) = close_timer();
        cmds.push(cmd);

        let amplification_sink = config::amplification_sink();
        let amplification_source = config::amplification_source();

        (
            Self {
                id,
                params,
                timer_abort,
                margin: (0, 0, 48, 0),
                amplification_sink,
                amplification_source,
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

    fn max_value(&self) -> f32 {
        match self.params {
            Params::SinkVolume(_, _) => {
                if self.amplification_sink {
                    150.0
                } else {
                    100.0
                }
            }
            Params::SourceVolume(_, _) => {
                if self.amplification_source {
                    150.0
                } else {
                    100.0
                }
            }
            _ => 100.0,
        }
    }

    pub fn view(&self) -> Element<'_, Msg> {
        let icon = widget::icon::from_name(self.params.icon_name());

        // Use large radius on value-OSD to enforce pill-shape with "Round" system style
        let radius;

        let osd_contents = if let Some(value) = self.params.value() {
            radius = cosmic::theme::active().cosmic().radius_l();
            let max_value = self.max_value();
            let osd_bar = if max_value > 100.0 {
                iced::widget::row![
                    widget::progress_bar(0.0..=100.0, value as f32)
                        .height(4)
                        .width(Length::Fixed(178.0)),
                    widget::progress_bar(100.0..=max_value, value as f32)
                        .height(4)
                        .width(Length::Fixed(89.0)),
                ]
                .apply(Element::from)
            } else {
                widget::progress_bar(0.0..=max_value, value as f32)
                    .height(4)
                    .width(Length::Fixed(267.0))
                    .apply(Element::from)
            };
            widget::container(
                iced::widget::row![
                    widget::container(icon.size(20))
                        .width(Length::Fixed(32.0))
                        .align_x(Alignment::Center),
                    widget::text::body(format!("{}%", value))
                        .width(Length::Fixed(32.0))
                        .align_x(Alignment::Center),
                    widget::horizontal_space().width(Length::Fixed(8.0)),
                    osd_bar,
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
        }
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .class(cosmic::theme::Container::custom(move |theme| {
            widget::container::Style {
                text_color: Some(theme.cosmic().on_bg_color().into()),
                background: Some(iced::Color::from(theme.cosmic().bg_color()).into()),
                border: Border {
                    radius: radius.into(),
                    width: 1.0,
                    color: theme.cosmic().bg_divider().into(),
                },
                shadow: Default::default(),
                icon_color: Some(theme.cosmic().on_bg_color().into()),
            }
        }));

        let osd_contents = if self.margin.0 != 0 || self.margin.2 != 0 {
            Element::from(widget::column::with_children(vec![
                vertical_space().height(self.margin.0 as f32).into(),
                osd_contents.into(),
                vertical_space().height(self.margin.2 as f32).into(),
            ]))
        } else {
            osd_contents.into()
        };
        let osd_contents = if self.margin.1 != 0 || self.margin.3 != 0 {
            Element::from(widget::row::with_children(vec![
                horizontal_space().width(self.margin.1 as f32).into(),
                osd_contents.into(),
                horizontal_space().width(self.margin.3 as f32).into(),
            ]))
        } else {
            osd_contents.into()
        };
        widget::autosize::autosize(
            widget::container(osd_contents)
                .align_x(Alignment::Center)
                .width(Length::Shrink)
                .align_bottom(Length::Shrink),
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
