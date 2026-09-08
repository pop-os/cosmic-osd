use cosmic_config::{Config, ConfigGet};

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";
const AMPLIFICATION_SOURCE: &str = "amplification_source";

const OSD_CONFIG: &str = "com.system76.CosmicOsd";
const SHOW_KEYBOARD_LAYOUT_OSD: &str = "show_keyboard_layout_osd";

pub fn amplification_sink() -> bool {
    Config::new(AUDIO_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(AMPLIFICATION_SINK).ok())
        .unwrap_or(true)
}

pub fn amplification_source() -> bool {
    Config::new(AUDIO_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(AMPLIFICATION_SOURCE).ok())
        .unwrap_or(false)
}

pub fn show_keyboard_layout_osd() -> bool {
    Config::new(OSD_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(SHOW_KEYBOARD_LAYOUT_OSD).ok())
        .unwrap_or(true)
}
