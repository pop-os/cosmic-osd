// XXX error handling?

use iced::futures::{FutureExt, StreamExt};

pub fn subscription(connection: zbus::Connection) -> iced::Subscription<Event> {
    iced::subscription::run(
        "dbus-service",
        async move {
            let settings_daemon = match CosmicSettingsDaemonProxy::new(&connection).await {
                Ok(value) => value,
                Err(_err) => iced::futures::future::pending().await,
            };
            let mut stream = settings_daemon.receive_display_brightness_changed().await;
            stream.filter_map(
                |evt| async move { Some(Event::DisplayBrightness(evt.get().await.ok()?)) },
            )
        }
        .flatten_stream(),
    )
}

#[derive(Debug)]
pub enum Event {
    DisplayBrightness(i32),
}

#[zbus::dbus_proxy(
    default_service = "com.system76.CosmicSettingsDaemon",
    interface = "com.system76.CosmicSettingsDaemon",
    default_path = "/com/system76/CosmicSettingsDaemon"
)]
trait CosmicSettingsDaemon {
    #[dbus_proxy(property)]
    fn display_brightness(&self) -> zbus::Result<i32>;
}

/*
pub async fn monitor(connection: &zbus::Connection) -> zbus::Result<()> {
    let settings_daemon = CosmicSettingsDaemonProxy::new(connection).await?;
    let mut stream = settings_daemon.receive_display_brightness_changed().await;
    while let Some(evt) = stream.next().await {
        let value = evt.get().await?;
        eprintln!("Value: {}", value);
    }
    Ok(())
}
*/
