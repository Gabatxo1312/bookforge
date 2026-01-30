use askama::Template;
use askama_web::WebTemplate;
use axum::response::{IntoResponse, Response};
use log::error;
use snafu::prelude::*;

use crate::{
    models::{book::BookError, user::UserError},
    routes::template_ctx::TemplateCtx,
    state::config::ConfigError,
};

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
    #[snafu(display("Migration Error"))]
    Migration {
        source: sea_orm::error::DbErr,
    },
    #[snafu(display("User Model Error"))]
    User {
        source: UserError,
    },
    #[snafu(display("Book Model Error"))]
    Book {
        source: BookError,
    },
    #[snafu(display("CSV Error"))]
    CSV {
        source: csv::Error,
    },
    #[snafu(display("IO Error"))]
    IO {
        source: std::io::Error,
    },
}

#[derive(Template, WebTemplate)]
#[template(path = "error.html")]
struct ErrorTemplate {
    state: AppStateErrorContext,
    ctx: TemplateCtx,
}

struct AppStateErrorContext {
    pub errors: Vec<AppStateError>,
}

impl From<AppStateError> for AppStateErrorContext {
    fn from(e: AppStateError) -> Self {
        error!("{:?}", e);

        Self { errors: vec![e] }
    }
}

impl IntoResponse for AppStateError {
    fn into_response(self) -> Response {
        let error_context = AppStateErrorContext::from(self);
        ErrorTemplate {
            state: error_context,
            ctx: TemplateCtx {
                base_path: "".to_string(),
            },
        }
        .into_response()
    }
}
