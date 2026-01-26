use snafu::prelude::*;
use tokio::fs::read_to_string;

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

#[derive(Snafu, Debug)]
pub enum ConfigError {
    #[snafu(display("File doesn't exist at path : {path}"))]
    FailedReadConfig {
        path: Utf8PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Failed parse config : {path}"))]
    FailedParseConfig {
        path: Utf8PathBuf,
        source: toml::de::Error,
    },
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AppConfig {
    #[serde(default = "AppConfig::default_sqlite_path")]
    pub database_path: Utf8PathBuf,
}

impl AppConfig {
    pub async fn new() -> Result<Self, ConfigError> {
        // TODO: Remove this
        let path = Utf8PathBuf::from("/home/torrpenn/projects/Gabatxo1312/bookforge/config.toml");

        let content = read_to_string(&path).await.context(FailedReadConfigSnafu {
            path: path.to_path_buf(),
        })?;

        toml::from_str(&content).context(FailedParseConfigSnafu {
            path: path.to_path_buf(),
        })
    }

    pub fn xdg_base_directories() -> BaseDirectories {
        BaseDirectories::with_prefix("bookforge")
    }

    pub fn default_sqlite_path() -> Utf8PathBuf {
        let config_dir = Self::xdg_base_directories().get_config_home().unwrap();

        Utf8PathBuf::from_path_buf(config_dir).unwrap()
    }
}
