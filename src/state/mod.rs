use sea_orm::{Database, DatabaseConnection};
use snafu::prelude::*;

use crate::{migrations::Migrator, state::config::AppConfig};
use error::*;
use sea_orm_migration::MigratorTrait;

pub mod api_config;
pub mod config;
pub mod error;
pub mod listener;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Result<Self, AppStateError> {
        log::info!("Load configurations...");
        let config: AppConfig = AppConfig::new().await.context(ConfigSnafu)?;

        let db: DatabaseConnection =
            Database::connect(format!("sqlite:{}?mode=rwc", &config.database_path))
                .await
                .context(SqliteSnafu)?;

        log::info!("Database Loaded at : {}", config.database_path.clone());

        Migrator::up(&db, None).await.context(MigrationSnafu)?;

        Ok(Self { config, db })
    }
}
