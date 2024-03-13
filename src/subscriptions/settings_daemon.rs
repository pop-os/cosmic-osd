// XXX error handling?

use cosmic::iced::{
    self,
    futures::{self, FutureExt, StreamExt},
};

pub fn subscription(connection: zbus::Connection) -> iced::Subscription<Event> {
    iced::subscription::run_with_id(
        "settings-daemon",
        async move {
            let settings_daemon = match CosmicSettingsDaemonProxy::new(&connection).await {
                Ok(value) => value,
                Err(_err) => iced::futures::future::pending().await,
            };
            let kb_stream = settings_daemon
                .receive_keyboard_brightness_changed()
                .await
                .filter_map(
                    |evt| async move { Some(Event::KeyboardBrightness(evt.get().await.ok()?)) },
                );
            let disp_stream = settings_daemon
                .receive_display_brightness_changed()
                .await
                .filter_map(
                    |evt| async move { Some(Event::DisplayBrightness(evt.get().await.ok()?)) },
                );
            futures::stream::select(kb_stream, disp_stream)
        }
        .flatten_stream(),
    )
}

#[derive(Debug)]
pub enum Event {
    DisplayBrightness(i32),
    KeyboardBrightness(i32),
}

#[zbus::proxy(
    default_service = "com.system76.CosmicSettingsDaemon",
    interface = "com.system76.CosmicSettingsDaemon",
    default_path = "/com/system76/CosmicSettingsDaemon"
)]
trait CosmicSettingsDaemon {
    #[zbus(property)]
    fn display_brightness(&self) -> zbus::Result<i32>;
    #[zbus(property)]
    fn keyboard_brightness(&self) -> zbus::Result<i32>;
}
