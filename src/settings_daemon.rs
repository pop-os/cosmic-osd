use futures::stream::StreamExt;

#[zbus::dbus_proxy(
    default_service = "com.system76.CosmicSettingsDaemon",
    interface = "com.system76.CosmicSettingsDaemon",
    default_path = "/com/system76/CosmicSettingsDaemon"
)]
trait CosmicSettingsDaemon {
    #[dbus_proxy(property)]
    fn display_brightness(&self) -> zbus::Result<i32>;
}

pub async fn monitor(connection: &zbus::Connection) -> zbus::Result<()> {
    let settings_daemon = CosmicSettingsDaemonProxy::new(connection).await?;
    let mut stream = settings_daemon.receive_display_brightness_changed().await;
    while let Some(evt) = stream.next().await {
        let value = evt.get().await?;
        eprintln!("Value: {}", value);
    }
    Ok(())
}
