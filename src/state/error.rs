use askama::Template;
use askama_web::WebTemplate;
use axum::response::{IntoResponse, Response};
use snafu::prelude::*;

use crate::state::config::ConfigError;

#[derive(Template, WebTemplate)]
#[template(path = "error.html")]
struct ErrorTemplate {}

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum AppStateError {
    Error,
    #[snafu(display("Sqlite Error"))]
    Sqlite {
        source: sea_orm::error::DbErr,
    },
    #[snafu(display("Config Error"))]
    ConfigError {
        source: ConfigError,
    },
}

impl IntoResponse for AppStateError {
    fn into_response(self) -> Response {
        ErrorTemplate {}.into_response()
    }
}
