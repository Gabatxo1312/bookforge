use bookforge::state::error::AppStateError;
use bookforge::state::listener::ListenerError;
use snafu::ErrorCompat;
use snafu::prelude::*;

use bookforge::build_app;
use bookforge::state::AppState;

#[derive(Snafu, Debug)]
pub enum AppError {
    #[snafu(display("Failed to initialize AppState"))]
    State {
        source: AppStateError,
    },
    #[snafu(display("Listener Error"))]
    Listener {
        source: ListenerError,
    },
    Error,
}

async fn main_inner() -> Result<(), AppError> {
    pretty_env_logger::init();
    let app_state = AppState::new().await.context(StateSnafu)?;

    let app = build_app(app_state.clone());

    let addr = app_state
        .config
        .listener
        .socket_addr()
        .context(ListenerSnafu)?;
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let _ = axum::serve(listener, app).await;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(errors) = main_inner().await {
        for error in errors.iter_chain() {
            println!("{}", error);
        }

        std::process::exit(1);
    }
}
