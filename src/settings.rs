use std::path::PathBuf;
use config::{Config, ConfigError, File};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Logging {
    pub level: String,
    pub file: PathBuf,
}
impl Default for Logging {
    fn default() -> Self {
        const DEFAULT_LEVEL: &str = "info";
        const DEFAULT_OUTPUT_FILE: &str = "walpurgis.log";

        Self {
            level: DEFAULT_LEVEL.into(),
            file: DEFAULT_OUTPUT_FILE.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    pub root: PathBuf,
}
impl Default for Assets {
    fn default() -> Self {
        const DEFAULT_ASSET_ROOT: &str = "data";
        Self {
            root: DEFAULT_ASSET_ROOT.into(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub logging: Logging,
    pub assets: Assets,
}

pub fn load() -> Result<Settings, ConfigError> {
    const CFG_PATH: &str = "walpurgis.toml";

    log::info!("Reading configuration file `{}`.", CFG_PATH);
    let cfg = File::with_name(CFG_PATH).required(false);

    let mut s = Config::default();
    s.merge(cfg)?;
    s.try_into()
}
