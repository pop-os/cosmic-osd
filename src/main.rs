#[macro_use]
extern crate tracing;

mod components;
mod localize;
mod subscriptions;

fn setup_logs() {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(format!(
        "warn,{}=warn",
        env!("CARGO_CRATE_NAME")
    )));

    if let Ok(journal_layer) = tracing_journald::layer() {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .with(journal_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
    }
}

fn main() {
    setup_logs();
    localize::localize();

    components::app::main().unwrap();
}
