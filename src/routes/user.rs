use std::collections::HashMap;

use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Form,
    extract::{Path, Query},
    response::Redirect,
};
use serde::Deserialize;
use serde_with::{NoneAsEmptyString, serde_as};
use snafu::prelude::*;

use crate::{
    models::{
        book::BookOperator,
        user::{self, UserOperator},
    },
    routes::router::Router,
    state::{
        context::AppStateContext,
        error::{AppStateError, BookSnafu, UserSnafu},
    },
};

#[derive(Template, WebTemplate)]
#[template(path = "users/index.html")]
struct UsersIndexTemplate {
    users_with_books_number: Vec<UserWithBookNumber>,
    query: IndexQuery,
    router: Router,
}

pub struct UserWithBookNumber {
    /// the user model
    pub user: user::Model,
    /// the number of books owned by this user
    pub owner_book_number: usize,
    /// the number of books borrowed by this user
    pub borrowed_book_number: usize,
}

#[serde_as]
#[derive(Deserialize, Clone)]
pub struct IndexQuery {
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    pub name: Option<String>,
}

pub async fn index(
    context: AppStateContext,
    Query(query): Query<IndexQuery>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let users = UserOperator::new(context.state.clone())
        .all_filtered(query.clone())
        .await
        .context(UserSnafu)?;

    let books = BookOperator::new(context.state.clone())
        .all()
        .await
        .context(BookSnafu)?;

    let mut result: Vec<UserWithBookNumber> = Vec::with_capacity(users.len());

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

        result.push(UserWithBookNumber {
            user,
            owner_book_number: *owner_books_size,
            borrowed_book_number: *borrowed_books_size,
        });
    }

    Ok(UsersIndexTemplate {
        users_with_books_number: result,
        query,
        router: context.router,
    })
}

#[derive(Deserialize)]
pub struct UserForm {
    pub name: String,
}

pub async fn create(
    context: AppStateContext,
    Form(form): Form<UserForm>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _ = UserOperator::new(context.state)
        .create(form)
        .await
        .context(UserSnafu)?;

    Ok(Redirect::to(&context.router.index_user_path()))
}

pub async fn update(
    context: AppStateContext,
    Path(id): Path<i32>,
    Form(form): Form<UserForm>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _ = UserOperator::new(context.state)
        .update(id, form)
        .await
        .context(UserSnafu)?;

    Ok(Redirect::to(&context.router.index_user_path()))
}

pub async fn delete(
    context: AppStateContext,
    Path(id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let _user = UserOperator::new(context.state)
        .delete(id)
        .await
        .context(UserSnafu)?;

    Ok(Redirect::to(&context.router.index_user_path()))
}

#[derive(Template, WebTemplate)]
#[template(path = "users/edit.html")]
struct EditTemplate {
    user: user::Model,
    router: Router,
}

pub async fn edit(
    context: AppStateContext,
    Path(id): Path<i32>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let user = UserOperator::new(context.state.clone())
        .find_by_id(id)
        .await
        .context(UserSnafu)?;

    Ok(EditTemplate {
        user,
        router: context.router,
    })
}

#[derive(Template, WebTemplate)]
#[template(path = "users/new.html")]
struct NewTemplate {
    router: Router,
}

pub async fn new(context: AppStateContext) -> impl axum::response::IntoResponse {
    NewTemplate {
        router: context.router,
    }
}
