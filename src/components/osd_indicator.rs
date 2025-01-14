// TODO: animation to fade in/out?
// TODO: Dismiss on click?

use cosmic::{
    iced::{self, window::Id as SurfaceId, Alignment, Border, Length},
    iced_runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
    iced_winit::commands::{
        layer_surface::{
            destroy_layer_surface, get_layer_surface, Anchor, KeyboardInteractivity, Layer,
        },
        overlap_notify::overlap_notify,
    },
    widget::{self, horizontal_space, vertical_space},
    Element, Task,
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
    pub margin: (i32, i32, i32, i32),
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
        (
            Self {
                id,
                params,
                timer_abort,
                margin: (0, 0, 48, 0),
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
