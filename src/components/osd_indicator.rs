// TODO: animation to fade in/out?
// Don't need msg; only state and view?
// - when to remove? timer subscription? Then need update.
// - oh, also want to dismiss on click?
// Never type

// Don't want just a widget, because it should have logic to create/destroy surface

use cosmic::{
    iced::{
        self,
        wayland::{
            actions::layer_surface::IcedMargin,
            layer_surface::{Anchor, KeyboardInteractivity, Layer},
        },
        widget, Border, Command, Subscription,
    },
    iced_runtime::{
        command::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
        window::Id as SurfaceId,
    },
    iced_sctk::commands::layer_surface::{destroy_layer_surface, get_layer_surface},
    theme, Element, Renderer,
};

use std::{collections::HashMap, time::Duration};
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Params {
    DisplayBrightness(i32),
    SinkMute(bool),
    SinkVolume(u32),
}

#[derive(Debug)]
pub enum Msg {
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
        // TODO: show as percent
        let label = match &self.params {
            Params::DisplayBrightness(brightness) => format!("Display brightness {}", brightness),
            Params::SinkMute(mute) => format!("Sink mute: {:?}", mute),
            Params::SinkVolume(volume) => format!("Sink volume: {}%", volume),
        };
        widget::container::Container::new(widget::row![iced::widget::text(label)])
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

    pub fn update(mut self, msg: Msg) -> (Option<Self>, Command<Msg>) {
        log::trace!("indicator msg: {:?}", msg);
        match msg {
            Msg::Close => (None, destroy_layer_surface(self.id)),
        }
    }
}

// TODO: Iced sctk output handling; option to have no initial surface
