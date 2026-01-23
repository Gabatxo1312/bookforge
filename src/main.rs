use bookforge::build_app;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = build_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    let _ = axum::serve(listener, app).await;
}
