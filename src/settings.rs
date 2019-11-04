use config::{Config, ConfigError, File};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Logging {
    pub level: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    pub arena_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub logging: Logging,
    pub assets: Assets,
}

pub fn read() -> Result<Settings, ConfigError> {
    let mut s = Config::new();
    s.merge(File::with_name("config/dev.toml").required(false))?;
    s.try_into()
}
