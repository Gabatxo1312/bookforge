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

/// Query for filter search query
#[serde_as]
#[derive(Deserialize, Clone, Debug)]
pub struct IndexQuery {
    pub title: Option<String>,
    pub page: Option<usize>,
    pub authors: Option<String>,
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    pub owner_id: Option<i32>,
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    pub current_holder_id: Option<i32>,
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct BookIndexTemplate {
    books_with_user: Vec<BookWithUser>,
    query: IndexQuery,
    users: Vec<UserModel>,
    current_page: u64,
    total_page: u64,
    base_query: String,
}

pub async fn index(
    State(state): State<AppState>,
    Query(query): Query<IndexQuery>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let page: u64 = query
        .page
        .map(|p| p.max(1) as u64) // Minimum 1
        .unwrap_or(1);

    // Get all Users
    let users = UserOperator::new(state.clone())
        .all()
        .await
        .context(UserSnafu)?;

    // Get all Book filtered with query
    let books_paginate = BookOperator::new(state)
        .list_paginate(page, Some(query.clone()))
        .await
        .context(BookSnafu)?;

    // Mapping between an user_id and user used in result to
    // get easily user with his id
    let user_by_id: HashMap<i32, UserModel> = users
        .clone()
        .into_iter()
        .map(|user| (user.id, user))
        .collect();

    // Build object of Book with his relation Owner (User) and current_holder (User)
    let result: Vec<BookWithUser> = books_paginate
        .books
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

    // build original search to be sure to keep
    // search when we change page
    let mut base_query = String::new();
    if let Some(title) = &query.title {
        base_query.push_str(&format!("title={}&", title));
    }
    if let Some(authors) = &query.authors {
        base_query.push_str(&format!("authors={}&", authors));
    }
    if let Some(owner_id) = &query.owner_id {
        base_query.push_str(&format!("owner_id={}&", owner_id));
    }
    if let Some(current_holder_id) = &query.current_holder_id {
        base_query.push_str(&format!("current_holder_id={}&", current_holder_id));
    }

    Ok(BookIndexTemplate {
        books_with_user: result,
        query,
        users,
        current_page: books_paginate.current_page,
        total_page: books_paginate.total_page,
        base_query,
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

/// Form to build a new book or an update
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
    let users = UserOperator::new(state).all().await.context(UserSnafu)?;

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
        .all()
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
