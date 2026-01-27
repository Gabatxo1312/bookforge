use std::collections::HashMap;

use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form,
    extract::{Path, State},
    response::Redirect,
};
use serde::Deserialize;
use snafu::prelude::*;

use crate::{
    models::{
        book::BookOperator,
        user::{self, UserOperator},
    },
    state::{
        AppState,
        error::{AppStateError, BookSnafu, UserSnafu},
    },
};

#[derive(Template, WebTemplate)]
#[template(path = "users/index.html")]
struct UsersIndexTemplate {
    user_with_books_number: Vec<(user::Model, usize, usize)>,
}

pub async fn index(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let users = UserOperator::new(state.clone())
        .list()
        .await
        .context(UserSnafu)?;

    let books = BookOperator::new(state.clone())
        .list()
        .await
        .context(BookSnafu)?;

    let mut result: Vec<(user::Model, usize, usize)> = vec![];

    let mut owner_books: HashMap<i32, usize> = HashMap::new();
    let mut borrowed_books: HashMap<i32, usize> = HashMap::new();

    for book in &books {
        *owner_books.entry(book.owner_id).or_default() += 1;
        if let Some(current_holder_id) = book.current_holder_id {
            *borrowed_books.entry(current_holder_id).or_default() += 1;
        }
    }

    for user in users {
        let owner_books_size = owner_books.get(&user.id).unwrap_or(&0);
        let borrowed_books_size = borrowed_books.get(&user.id).unwrap_or(&0);

        result.push((user, *owner_books_size, *borrowed_books_size));
    }

    Ok(UsersIndexTemplate {
        user_with_books_number: result,
    })
}

#[derive(Deserialize)]
pub struct UserForm {
    pub name: String,
}

pub async fn create(
    State(state): State<AppState>,
    Form(form): Form<UserForm>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _user = UserOperator::new(state)
        .create(form)
        .await
        .context(UserSnafu)?;

    Ok(Redirect::to("/users"))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _user = UserOperator::new(state)
        .delete(id)
        .await
        .context(UserSnafu)?;

    Ok(Redirect::to("/users"))
}
