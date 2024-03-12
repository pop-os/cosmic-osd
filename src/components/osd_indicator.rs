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

use std::time::Duration;

#[derive(Debug)]
pub enum Params {
    DisplayBrightness(i32),
    SinkMute(bool),
    SinkVolume(u32),
    AirplaneMode(bool),
}

impl Params {
    fn icon_name(&self) -> &'static str {
        match self {
            Self::DisplayBrightness(_) => "display-brightness-symbolic",
            Self::AirplaneMode(true) => "airplane-mode-symbolic",
            Self::AirplaneMode(false) => "airplane-mode-disabled-symbolic",
            // TODO audio-volume-low-symbolic, audio-volume-high-symbolic
            Self::SinkVolume(_) => "audio-volume-medium-symbolic",
            // XXX false?
            Self::SinkMute(_) => "audio-volume-muted-symbolic",
        }
    }

    fn value(&self) -> Option<u32> {
        match self {
            Self::DisplayBrightness(value) => Some(*value as u32),
            Self::SinkVolume(value) => Some(*value),
            Self::SinkMute(_) | Self::AirplaneMode(_) => None,
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
}

impl State {
    pub fn new(id: SurfaceId, params: Params) -> (Self, Command<Msg>) {
        // Anchor to bottom right, with margin?
        let mut cmds = vec![];
        cmds.push(get_layer_surface(SctkLayerSurfaceSettings {
            id,
            keyboard_interactivity: KeyboardInteractivity::None,
            namespace: "osd".into(),
            layer: Layer::Overlay,
            // XXX size of window?
            size: Some((Some(300), Some(100))),
            anchor: Anchor::BOTTOM,
            margin: IcedMargin {
                bottom: 10,
                ..Default::default()
            },
            ..Default::default()
        }));
        cmds.push(Command::perform(
            async {
                let duration = Duration::from_secs(5);
                tokio::time::sleep(duration).await;
            },
            |_| Msg::Close,
        ));
        (Self { id, params }, Command::batch(cmds))
    }

    pub fn view(&self) -> Element<'_, Msg> {
        let icon = cosmic::widget::icon::from_name(self.params.icon_name());
        // TODO if value is None, large icon
        // TODO: show as percent
        let row = if let Some(value) = self.params.value() {
            let slider = cosmic::widget::slider(0..=100, value, |_| Msg::Ignore);
            widget::row![icon, iced::widget::text(format!("{}", value)), slider]
        } else {
            widget::row![icon]
        };
        widget::container::Container::new(row)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(cosmic::theme::Container::custom(|theme| {
                cosmic::iced_style::container::Appearance {
                    text_color: Some(theme.cosmic().on_bg_color().into()),
                    background: Some(iced::Color::from(theme.cosmic().background.base).into()),
                    border: Border {
                        radius: (12.0).into(),
                        width: 0.0,
                        color: iced::Color::TRANSPARENT,
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

// TODO: Iced sctk output handling; option to have no initial surface
