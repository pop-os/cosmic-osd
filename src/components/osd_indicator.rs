// TODO: animation to fade in/out?
// TODO: Dismiss on click?

use cosmic::{
    iced::{
        self,
        wayland::{
            actions::layer_surface::IcedMargin,
            layer_surface::{Anchor, KeyboardInteractivity, Layer},
        },
        widget, Border, Command,
    },
    iced_runtime::{
        command::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
        window::Id as SurfaceId,
    },
    iced_sctk::commands::layer_surface::{destroy_layer_surface, get_layer_surface},
    Element,
};
use futures::future::{abortable, AbortHandle, Aborted};

use std::time::Duration;

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

fn close_timer() -> (Command<Msg>, AbortHandle) {
    let (future, timer_abort) = abortable(async {
        let duration = Duration::from_secs(3);
        tokio::time::sleep(duration).await;
    });
    let command = Command::perform(future, |res| {
        if res == Err(Aborted) {
            Msg::Ignore
        } else {
            Msg::Close
        }
    });
    (command, timer_abort)
}

impl State {
    pub fn new(id: SurfaceId, params: Params) -> (Self, Command<Msg>) {
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
            Command::batch(cmds),
        )
    }

    // Re-use OSD surface to show a different OSD
    // Resets close timer
    pub fn replace_params(&mut self, params: Params) -> Command<Msg> {
        self.params = params;
        // Reset timer
        self.timer_abort.abort();
        let (cmd, timer_abort) = close_timer();
        self.timer_abort = timer_abort;
        cmd
    }

    pub fn view(&self) -> Element<'_, Msg> {
        let icon = cosmic::widget::icon::from_name(self.params.icon_name()).size(24);

        let row = if let Some(value) = self.params.value() {
            widget::row![
                cosmic::widget::Container::new(icon)
                    .width(iced::Length::FillPortion(1))
                    .align_x(iced::alignment::Horizontal::Center),
                cosmic::widget::progress_bar(0. ..=100., value as f32)
                    .height(8)
                    .width(iced::Length::FillPortion(5)),
                iced::widget::text(format!("{}%", value))
                    .size(16)
                    .width(iced::Length::FillPortion(1))
                    .horizontal_alignment(iced::alignment::Horizontal::Right)
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
            .width(340)
        } else {
            widget::row![icon.size(48)]
        };
        widget::container::Container::new(row)
            .padding(16)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .style(cosmic::theme::Container::custom(|theme| {
                cosmic::iced_style::container::Appearance {
                    text_color: Some(theme.cosmic().on_bg_color().into()),
                    background: Some(iced::Color::from(theme.cosmic().background.base).into()),
                    border: Border {
                        radius: theme.cosmic().radius_m().into(),
                        width: 1.0,
                        color: theme.cosmic().bg_divider().into(),
                    },
                    shadow: Default::default(),
                    icon_color: Some(theme.cosmic().on_bg_color().into()),
                }
            }))
            .into()
    }

    pub fn update(self, msg: Msg) -> (Option<Self>, Command<Msg>) {
        log::trace!("indicator msg: {:?}", msg);
        match msg {
            Msg::Ignore => (Some(self), Command::none()),
            Msg::Close => (None, destroy_layer_surface(self.id)),
        }
    }
}
