use cosmic::{
    iced::{self, event::listen_with, keyboard::Key},
    iced_core::keyboard,
};

#[derive(Clone, Debug)]
pub enum Msg {
    BrightnessDown,
    BrightnessUp,
    AudioVolumeDown,
    AudioVolumeUp,
    AudioVolumeMute,
}

pub fn subscription() -> iced::Subscription<Msg> {
    listen_with(|event, status| match status {
        iced::event::Status::Ignored => match event {
            iced::Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(named),
                ..
            }) => match named {
                keyboard::key::Named::BrightnessDown => Some(Msg::BrightnessDown),
                keyboard::key::Named::BrightnessUp => Some(Msg::BrightnessUp),
                keyboard::key::Named::AudioVolumeDown => Some(Msg::AudioVolumeDown),
                keyboard::key::Named::AudioVolumeUp => Some(Msg::AudioVolumeUp),
                keyboard::key::Named::AudioVolumeMute => Some(Msg::AudioVolumeMute),
                _ => None,
            },
            _ => None,
        },
        iced::event::Status::Captured => None,
    })
}
