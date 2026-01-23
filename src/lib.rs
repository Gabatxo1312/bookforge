use axum::{Router, routing::get};
use static_serve::embed_assets;

mod error;
mod routes;

pub fn build_app() -> Router {
    embed_assets!("assets", compress = true);

    Router::new()
        .route("/", get(routes::book::index))
        .route("/books/{id}", get(routes::book::show))
        .route("/books/{id}/edit", get(routes::book::edit))
        .route("/books/new", get(routes::book::new))
        .route("/users", get(routes::user::index))
        .nest("/assets", static_router())
}
