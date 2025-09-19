use cosmic_config::{Config, ConfigGet};

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";
const AMPLIFICATION_SOURCE: &str = "amplification_source";

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
