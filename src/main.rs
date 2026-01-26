use bookforge::state::error::AppStateError;
use snafu::prelude::*;

use bookforge::build_app;
use bookforge::state::AppState;

#[derive(Snafu, Debug)]
enum AppError {
    #[snafu(display("Failed to initialize AppState"))]
    State {
        source: AppStateError,
    },
    Error,
}

async fn main_inner() -> Result<(), AppError> {
    let app = build_app(AppState::new().await.context(StateSnafu)?);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    let _ = axum::serve(listener, app).await;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(_errors) = main_inner().await {
        // for error in errors.iter_chain() {
        //     println!("{}", error);
        // }

        std::process::exit(1);
    }
}
