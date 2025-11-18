// TODO: animation to fade in/out?
// TODO: Dismiss on click?

use crate::{components::app::DisplayMode, config};
use cosmic::{
    Apply, Element, Task,
    iced::{self, Alignment, Border, Length, window::Id as SurfaceId},
    iced_runtime::platform_specific::wayland::layer_surface::{
        IcedMargin, IcedOutput, SctkLayerSurfaceSettings,
    },
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
    DisplayBrightness(f64),        // Rung ratio k/20.0 (hotkeys)
    DisplayBrightnessExact(f64),   // Exact raw ratio raw/max (slider/arbitrary)
    DisplayToggle(DisplayMode),
    DisplayNumber(u32),
    KeyboardBrightness(f64),
    SinkVolume(u32, bool),
    SourceVolume(u32, bool),
    AirplaneMode(bool),
    TouchpadEnabled(TouchpadOverride),
}

impl Params {
    fn icon_name(&self) -> &'static str {
        match self {
            Self::DisplayBrightness(_) | Self::DisplayBrightnessExact(_) => "display-brightness-symbolic",
            Self::DisplayToggle(DisplayMode::All) => "laptop-symbolic",
            Self::DisplayToggle(DisplayMode::External) => "display-symbolic",
            Self::DisplayNumber(_) => {
                unreachable!("DisplayNumber uses custom rendering and should not call icon_name()")
            }
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
                let mut rung = (*value * 20.0).round() as u32;
                if rung > 20 { rung = 20; }
                if rung == 0 && *value > 0.0 {
                    Some(1) // 1% at the floor
                } else {
                    Some(5 * rung)
                }
            }

            // SLIDER / EXACT: show precise percent from exact ratio, with 1% floor.
            Self::DisplayBrightnessExact(value) => {
                // round(100 * ratio)
                let mut p = (*value * 100.0).round() as i32;
                if p <= 0 && *value >= 0.0 { p = 1; } // never show 0%
                if p > 100 { p = 100; }
                Some(p as u32)
            }
            Self::KeyboardBrightness(value) => Some((*value * 100.) as u32),
            Self::SinkVolume(_, true) => Some(0),
            Self::SourceVolume(_, true) => Some(0),
            Self::SinkVolume(value, false) => Some(*value),
            Self::SourceVolume(value, false) => Some(*value),
            Self::AirplaneMode(_) => None,
            Self::TouchpadEnabled(_) => None,
            Self::DisplayToggle(_) => None,
            Self::DisplayNumber(_) => None,
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
    let command = cosmic::task::future(async move {
        match future.await {
            Ok(_) => Msg::Close,
            Err(Aborted) => Msg::Ignore,
        }
    });
    (command, timer_abort)
}

/// Creates a 1-second timer for display identifiers
/// When the timer expires, it sends Msg::Close to remove the display identifier
fn display_identifier_timer() -> (Task<Msg>, AbortHandle) {
    let (future, timer_abort) = abortable(async {
        let duration = Duration::from_secs(1);
        tokio::time::sleep(duration).await;
    });
    let command = cosmic::task::future(async move {
        match future.await {
            Ok(_) => Msg::Close,
            Err(Aborted) => Msg::Ignore,
        }
    });
    (command, timer_abort)
}

impl State {
    pub fn new(id: SurfaceId, params: Params) -> (Self, Task<Msg>) {
        Self::new_with_output(id, params, IcedOutput::Active)
    }

    pub fn new_with_output(id: SurfaceId, params: Params, output: IcedOutput) -> (Self, Task<Msg>) {
        let mut cmds = vec![];

        let is_display_number = matches!(params, Params::DisplayNumber(_));
        let anchor = if is_display_number {
            Anchor::TOP
        } else {
            Anchor::BOTTOM
        };

        // For display numbers, set exclusive_zone to -1 so they don't block input
        // in transparent areas. For other OSDs, use default behavior.
        let exclusive_zone = if is_display_number { -1 } else { 0 };
        let margin = if is_display_number {
            // Set top margin for display identifiers
            IcedMargin {
                top: 48,
                right: 0,
                bottom: 0,
                left: 0,
            }
        } else {
            // No margin for other OSDs (they use widget-based margins)
            IcedMargin {
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
            }
        };

        cmds.push(get_layer_surface(SctkLayerSurfaceSettings {
            id,
            keyboard_interactivity: KeyboardInteractivity::None,
            namespace: "osd".into(),
            layer: Layer::Overlay,
            size: None,
            anchor,
            output,
            exclusive_zone,
            margin,
            ..Default::default()
        }));

        cmds.push(overlap_notify(id, true));

        // Display numbers auto-close after 1 second, other OSDs after 3 seconds
        let timer_abort = if is_display_number {
            let (cmd, timer_abort) = display_identifier_timer();
            cmds.push(cmd);
            timer_abort
        } else {
            let (cmd, timer_abort) = close_timer();
            cmds.push(cmd);
            timer_abort
        };

        let amplification_sink = config::amplification_sink();
        let amplification_source = config::amplification_source();

        // Margin: (top, right, bottom, left)
        // Display numbers at top, other OSDs at bottom
        let margin = if is_display_number {
            (48, 0, 0, 0) // Top margin for display numbers
        } else {
            (0, 0, 48, 0) // Bottom margin for other OSDs
        };

        (
            Self {
                id,
                params,
                timer_abort,
                margin,
                amplification_sink,
                amplification_source,
            },
            Task::batch(cmds),
        )
    }

    pub fn params(&self) -> &Params {
        &self.params
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

    // Reset the timer for display identifiers
    // This is called when a new identify message is received to keep them visible
    pub fn reset_display_identifier_timer(&mut self) -> Task<Msg> {
        if !matches!(self.params, Params::DisplayNumber(_)) {
            return Task::none();
        }

        self.timer_abort.abort();
        let (cmd, timer_abort) = display_identifier_timer();
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
        // Display numbers use a completely different rendering
        if let Params::DisplayNumber(display_number) = self.params {
            return self.view_display_number(display_number);
        }

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
                        .width(Length::FillPortion(2)),
                    widget::progress_bar(100.0..=max_value, value as f32)
                        .height(4)
                        .width(Length::FillPortion(1)),
                ]
                .width(Length::Fixed(266.0))
                .apply(Element::from)
            } else {
                widget::progress_bar(0.0..=max_value, value as f32)
                    .height(4)
                    .width(Length::Fixed(266.0))
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
                osd_contents,
                horizontal_space().width(self.margin.3 as f32).into(),
            ]))
        } else {
            osd_contents
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

    fn view_display_number(&self, display_number: u32) -> Element<'_, Msg> {
        const CONTAINER_BASE_SIZE: u16 = 27;
        const TEXT_SIZE: u16 = 45;

        let theme = cosmic::theme::active();
        let cosmic_theme = theme.cosmic();

        let number_text = widget::text::title1(format!("{}", display_number))
            .size(TEXT_SIZE)
            .line_height(cosmic::iced::widget::text::LineHeight::Absolute(
                cosmic::iced::Pixels(TEXT_SIZE as f32),
            ))
            .width(Length::Shrink)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        let content = widget::container(number_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        let padding = cosmic_theme.space_l();
        let square_size = (CONTAINER_BASE_SIZE + (padding * 2)) as f32;

        let container = widget::container(content)
            .padding(padding)
            .width(Length::Fixed(square_size))
            .height(Length::Fixed(square_size))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .class(cosmic::theme::Container::custom(move |theme| {
                widget::container::Style {
                    text_color: Some(iced::Color::from(theme.cosmic().on_accent_color()).into()),
                    background: Some(iced::Color::from(theme.cosmic().accent_color()).into()),
                    border: Border {
                        radius: theme.cosmic().radius_m().into(),
                        width: 0.0,
                        color: iced::Color::TRANSPARENT,
                    },
                    shadow: Default::default(),
                    icon_color: Some(iced::Color::from(theme.cosmic().on_accent_color()).into()),
                }
            }));

        let autosize_id = iced::id::Id::new(format!("display-number-{}", display_number));
        widget::autosize::autosize(container, autosize_id)
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
