mod polkit_agent;
mod settings_daemon;

fn main() {
    futures::executor::block_on(async {
        dbus_serve().await.unwrap();
    });
}

async fn dbus_serve() -> zbus::Result<()> {
    let connection = zbus::Connection::session().await?;
    connection.request_name("com.system76.CosmicOsd").await?;
    settings_daemon::monitor(&connection).await?;
    Ok(())
}
