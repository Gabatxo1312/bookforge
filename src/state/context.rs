use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    routes::router::Router,
    state::{AppState, error::AppStateError},
};

#[derive(Debug)]
pub struct AppStateContext {
    pub state: AppState,
    pub router: Router,
}

impl FromRequestParts<AppState> for AppStateContext {
    type Rejection = AppStateError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let router = Router {
            base_path: state.clone().config.base_path,
        };

        Ok(Self {
            state: state.clone(),
            router,
        })
    }
}
