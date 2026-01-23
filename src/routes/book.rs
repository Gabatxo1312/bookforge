use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Path;

use crate::error::AppStateError;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct BookIndexTemplate {}

pub async fn index() -> Result<impl axum::response::IntoResponse, AppStateError> {
    if 0 > 1 {
        return Err(AppStateError::Error);
    }

    Ok(BookIndexTemplate {})
}

#[derive(Template, WebTemplate)]
#[template(path = "books/show.html")]
struct ShowBookTemplate {}

pub async fn show(
    Path(_id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    if 0 > 1 {
        return Err(AppStateError::Error);
    }

    Ok(ShowBookTemplate {})
}

#[derive(Template, WebTemplate)]
#[template(path = "books/new.html")]
struct NewBookTemplate {}

pub async fn new() -> Result<impl axum::response::IntoResponse, AppStateError> {
    if 0 > 1 {
        return Err(AppStateError::Error);
    }

    Ok(NewBookTemplate {})
}

#[derive(Template, WebTemplate)]
#[template(path = "books/edit.html")]
struct EditBookTemplate {}

pub async fn edit(
    Path(_id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    if 0 > 1 {
        return Err(AppStateError::Error);
    }

    Ok(EditBookTemplate {})
}
