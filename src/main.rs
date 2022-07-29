mod polkit_agent;
mod settings_daemon;

use gtk4::glib;

fn main() {
    gtk4::init().unwrap();
    glib::MainContext::default().spawn(async {
        dbus_serve().await.unwrap();
    });
    glib::MainLoop::new(None, false).run();
}

async fn dbus_serve() -> zbus::Result<()> {
    let system_connection = zbus::ConnectionBuilder::system()?
        .internal_executor(false)
        .build()
        .await?;
    glib::MainContext::default().spawn(glib::clone!(@strong system_connection => async move {
       loop {
           system_connection.executor().tick().await;
       }
    }));

    let connection = zbus::ConnectionBuilder::session()?
        .internal_executor(false)
        .build()
        .await?;
    glib::MainContext::default().spawn(glib::clone!(@strong connection => async move {
       loop {
           connection.executor().tick().await;
       }
    }));

    connection.request_name("com.system76.CosmicOsd").await?;
    polkit_agent::register_agent(&system_connection).await?;
    settings_daemon::monitor(&connection).await?;
    Ok(())
}
