use sea_orm::{Database, DatabaseConnection};
use snafu::prelude::*;

use crate::state::config::AppConfig;
use error::*;

pub mod config;
pub mod error;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Result<Self, AppStateError> {
        let config: AppConfig = AppConfig::new().await.context(ConfigSnafu)?;

        let db: DatabaseConnection =
            Database::connect(format!("sqlite:{}?mode=rwc", &config.database_path))
                .await
                .context(SqliteSnafu)?;

        Ok(Self { config, db })
    }
}
