use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    routing::{get, post},
};
use static_serve::embed_assets;

use crate::state::AppState;

mod migrations;
mod models;
mod routes;
pub mod state;

pub fn build_app(state: AppState) -> Router {
    embed_assets!("assets", compress = true);

    Router::new()
        .route("/", get(routes::book::index))
        .route("/books/new", get(routes::book::new))
        .route("/books", post(routes::book::create))
        .route("/books/{id}", get(routes::book::show))
        .route("/books/{id}", post(routes::book::update))
        .route("/books/{id}/delete", post(routes::book::delete))
        .route("/books/{id}/edit", get(routes::book::edit))
        .route("/books/download_csv", get(routes::book::download_csv))
        .route("/users", get(routes::user::index))
        .route("/users/new", get(routes::user::new))
        .route("/users/{id}/edit", get(routes::user::edit))
        .route("/users/{id}", post(routes::user::update))
        .route("/users", post(routes::user::create))
        .route("/users/{id}/delete", post(routes::user::delete))
        .nest("/assets", static_router())
        .fallback(error_handler)
        .with_state(state)
}

#[derive(Template, WebTemplate)]
#[template(path = "404.html")]
struct NotFoundTemplate {}

pub async fn error_handler() -> impl axum::response::IntoResponse {
    NotFoundTemplate {}
}
