use askama::Template;
use askama_web::WebTemplate;
use axum::response::{IntoResponse, Response};

#[derive(Template, WebTemplate)]
#[template(path = "error.html")]
struct ErrorTemplate {}

pub enum AppStateError {
    Error,
}

impl IntoResponse for AppStateError {
    fn into_response(self) -> Response {
        ErrorTemplate {}.into_response()
    }
}
