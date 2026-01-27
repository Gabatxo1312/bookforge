use axum::{
    Router,
    routing::{delete, get, post},
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
        .route("/books/{id}", get(routes::book::show))
        .route("/books/{id}/edit", get(routes::book::edit))
        .route("/books/new", get(routes::book::new))
        .route("/users", get(routes::user::index))
        .route("/users", post(routes::user::create))
        .route("/users/{id}", post(routes::user::delete))
        .nest("/assets", static_router())
        .with_state(state)
}
