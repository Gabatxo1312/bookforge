use std::collections::HashMap;

use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use serde_with::{NoneAsEmptyString, serde_as};
use snafu::prelude::*;

use crate::models::book::Model as BookModel;
use crate::models::user::Model as UserModel;

use crate::{
    models::{book::BookOperator, user::UserOperator},
    state::{
        AppState,
        error::{AppStateError, BookSnafu, UserSnafu},
    },
};

// Book list with the owner and the current holder inside
struct BookWithUser {
    pub book: BookModel,
    pub owner: UserModel,
    pub current_holder: Option<UserModel>,
}

#[serde_as]
#[derive(Deserialize, Clone)]
pub struct BookQuery {
    pub title: Option<String>,
    pub authors: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub owner_id: Option<i32>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub current_holder_id: Option<i32>,
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct BookIndexTemplate {
    books_with_user: Vec<BookWithUser>,
    query: BookQuery,
    users: Vec<UserModel>,
}

pub async fn index(
    State(state): State<AppState>,
    Query(query): Query<BookQuery>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let users = UserOperator::new(state.clone())
        .list()
        .await
        .context(UserSnafu)?;
    let books = BookOperator::new(state)
        .list(Some(query.clone()))
        .await
        .context(BookSnafu)?;

    let user_by_id: HashMap<i32, UserModel> = users
        .clone()
        .into_iter()
        .map(|user| (user.id, user))
        .collect();

    let result: Vec<BookWithUser> = books
        .into_iter()
        .filter_map(|book| {
            let owner = user_by_id.get(&book.owner_id).cloned()?;
            let current_holder = book
                .current_holder_id
                .and_then(|id| user_by_id.get(&id).cloned());

            Some(BookWithUser {
                book,
                owner,
                current_holder,
            })
        })
        .collect();

    Ok(BookIndexTemplate {
        books_with_user: result,
        query,
        users,
    })
}

#[derive(Template, WebTemplate)]
#[template(path = "books/show.html")]
struct ShowBookTemplate {
    book: BookModel,
    owner: UserModel,
    current_holder: Option<UserModel>,
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let book = BookOperator::new(state.clone())
        .find_by_id(id)
        .await
        .context(BookSnafu)?;

    let owner = UserOperator::new(state.clone())
        .find_by_id(book.owner_id)
        .await
        .context(UserSnafu)?;

    let current_holder: Option<UserModel> = if let Some(current_holder_id) = book.current_holder_id
    {
        Some(
            UserOperator::new(state.clone())
                .find_by_id(current_holder_id)
                .await
                .context(UserSnafu)?,
        )
    } else {
        None
    };

    Ok(ShowBookTemplate {
        book,
        owner,
        current_holder,
    })
}

#[serde_as]
#[derive(Deserialize)]
pub struct BookForm {
    pub title: String,
    pub authors: String,
    pub owner_id: i32,
    pub description: Option<String>,
    pub comment: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub current_holder_id: Option<i32>,
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<BookForm>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _ = BookOperator::new(state)
        .create(form)
        .await
        .context(BookSnafu)?;

    Ok(Redirect::to("/").into_response())
}

#[derive(Template, WebTemplate)]
#[template(path = "books/new.html")]
struct NewBookTemplate {
    users: Vec<UserModel>,
}

pub async fn new(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let users = UserOperator::new(state).list().await.context(UserSnafu)?;

    Ok(NewBookTemplate { users })
}

#[derive(Template, WebTemplate)]
#[template(path = "books/edit.html")]
struct EditBookTemplate {
    users: Vec<UserModel>,
    book: BookModel,
}

pub async fn edit(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let users = UserOperator::new(state.clone())
        .list()
        .await
        .context(UserSnafu)?;
    let book = BookOperator::new(state)
        .find_by_id(id)
        .await
        .context(BookSnafu)?;

    Ok(EditBookTemplate { users, book })
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<BookForm>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _ = BookOperator::new(state)
        .update(id, form)
        .await
        .context(BookSnafu)?;

    Ok(Redirect::to(&format!("/books/{}", id)).into_response())
}
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _ = BookOperator::new(state)
        .delete(id)
        .await
        .context(BookSnafu)?;

    Ok(Redirect::to("/").into_response())
}
