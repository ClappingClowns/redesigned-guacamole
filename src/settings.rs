use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub level: String,
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Assets {
    pub arena_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub logging: Logging,
    pub assets: Assets,
}

pub fn read() -> Result<Settings, ConfigError> {
    let mut s = Config::new();
    s.merge(File::with_name("config/dev.toml").required(false))?;
    s.try_into()
}