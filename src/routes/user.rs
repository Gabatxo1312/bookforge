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
    models::user::{self, UserOperator},
    state::{
        AppState,
        error::{AppStateError, UserSnafu},
    },
};

#[derive(Template, WebTemplate)]
#[template(path = "users/index.html")]
struct UsersIndexTemplate {
    users: Vec<user::Model>,
}

pub async fn index(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse, AppStateError> {
    let users = UserOperator::new(state).list().await.context(UserSnafu)?;

    Ok(UsersIndexTemplate { users })
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
