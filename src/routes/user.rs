use askama::Template;
use askama_web::WebTemplate;

use crate::error::AppStateError;

#[derive(Template, WebTemplate)]
#[template(path = "users/index.html")]
struct UsersIndexTemplate {}

pub async fn index() -> Result<impl axum::response::IntoResponse, AppStateError> {
    if 0 > 1 {
        return Err(AppStateError::Error);
    }

    Ok(UsersIndexTemplate {})
}
