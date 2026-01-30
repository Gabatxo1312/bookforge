use snafu::prelude::*;
use tokio::fs::read_to_string;

use camino::Utf8PathBuf;
use dirs::config_dir;
use serde::{Deserialize, Serialize};

use crate::state::listener::Listener;

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
    #[snafu(display("Failed parse config : {path}"))]
    IO {
        path: Utf8PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Config is empty: {path}"))]
    ConfigEmpty { path: Utf8PathBuf },
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AppConfig {
    #[serde(default = "AppConfig::default_sqlite_path")]
    pub database_path: Utf8PathBuf,
    pub locale: String,
    pub base_path: String,
    pub listener: Listener,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            database_path: Self::default_sqlite_path(),
            base_path: Self::default_base_path(),
            locale: Self::default_locale(),
            listener: Listener::default(),
        }
    }
}

impl AppConfig {
    pub async fn new() -> Result<Self, ConfigError> {
        let path = Self::config_file_path();

        let file_exist = tokio::fs::try_exists(&path)
            .await
            .context(IOSnafu { path: path.clone() })?;

        if file_exist {
            Self::parse(path).await
        } else {
            // Create parent and create an empty config file
            let parent_path = path.parent().unwrap();
            tokio::fs::create_dir_all(&parent_path)
                .await
                .context(IOSnafu { path: parent_path })?;
            tokio::fs::write(
                path.clone(),
                toml::to_string(&AppConfig::default()).unwrap(),
            )
            .await
            .context(IOSnafu { path: path.clone() })?;

            Ok(AppConfig::default())
        }
    }

    async fn parse(path: Utf8PathBuf) -> Result<Self, ConfigError> {
        let content = read_to_string(&path).await.context(FailedReadConfigSnafu {
            path: path.to_path_buf(),
        })?;

        toml::from_str(&content).context(FailedParseConfigSnafu {
            path: path.to_path_buf(),
        })
    }

    fn config_path() -> Utf8PathBuf {
        let mut config_dir = Utf8PathBuf::from_path_buf(config_dir().unwrap()).unwrap();
        config_dir.push("bookforge");

        return config_dir;
    }

    fn config_file_path() -> Utf8PathBuf {
        Self::config_path().join("BookForge.toml")
    }

    fn default_locale() -> String {
        "en".to_string()
    }

    fn default_base_path() -> String {
        "".to_string()
    }

    pub fn default_sqlite_path() -> Utf8PathBuf {
        Self::config_path().join("db.sqlite")
    }
}
