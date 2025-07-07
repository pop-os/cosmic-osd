// TODO: Handle loss of connection, name?

use cosmic::iced;
use futures::stream;

static NAME: &str = "com.system76.CosmicOsd";

#[derive(Clone, Debug)]
pub enum Event {
    Connection(zbus::Connection),
    SystemConnection(zbus::Connection),
    Error(&'static str, zbus::Error),
}

enum State {
    Start,
    CreatedConnection,
    CreatedSystemConnection,
}

pub fn subscription() -> iced::Subscription<Event> {
    iced::Subscription::run_with_id(
        "dbus-service",
        stream::unfold(State::Start, |state| async move {
            match state {
                State::Start => Some((
                    result_to_event(
                        connection().await,
                        "create session connection",
                        Event::Connection,
                    ),
                    State::CreatedConnection,
                )),
                State::CreatedConnection => Some((
                    result_to_event(
                        system_connection().await,
                        "create system connection",
                        Event::SystemConnection,
                    ),
                    State::CreatedSystemConnection,
                )),
                State::CreatedSystemConnection => iced::futures::future::pending().await,
            }
        }),
    )
}

async fn connection() -> zbus::Result<zbus::Connection> {
    zbus::connection::Builder::session()?
        .name(NAME)?
        .build()
        .await
}

async fn system_connection() -> zbus::Result<zbus::Connection> {
    zbus::connection::Builder::system()?.build().await
}

fn result_to_event<T>(res: zbus::Result<T>, context: &'static str, f: fn(T) -> Event) -> Event {
    match res {
        Ok(val) => f(val),
        Err(err) => Event::Error(context, err),
    }
}
