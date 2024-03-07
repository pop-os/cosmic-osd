use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DesktopLanguageRequester,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

mod components;
mod subscriptions;

pub static LANG_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| fluent_language_loader!());

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::LANG_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::LANG_LOADER, $message_id, $($args), *)
    }};
}

fn main() {
    let requested_languages = DesktopLanguageRequester::requested_languages();
    i18n_embed::select(&*LANG_LOADER, &Localizations, &requested_languages)
        .expect("Failed to load languages");

    env_logger::init();
    components::app::main().unwrap();
}
