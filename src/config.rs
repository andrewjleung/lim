use std::rc::Rc;

use crate::prelude::*;
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogDir(pub PathBuf);

impl Default for LogDir {
    fn default() -> Self {
        let data_dir = dirs::data_local_dir().expect("Data directory is defined");
        let data_dir = PathBuf::from_path_buf(data_dir).expect("Data directory is UTF-8");

        Self(data_dir.join("lim"))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigDir(pub PathBuf);

impl Default for ConfigDir {
    fn default() -> Self {
        let config_dir = dirs::config_dir().expect("Config directory is defined");
        let config_dir = PathBuf::from_path_buf(config_dir).expect("Config directory is UTF-8");

        Self(config_dir.join("lim"))
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub log_dir: Rc<LogDir>,
}

pub fn config() -> Result<Config> {
    let config_path = ConfigDir::default().0.to_path_buf();

    let config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(&config_path))
        .extract()?;

    Ok(config)
}
