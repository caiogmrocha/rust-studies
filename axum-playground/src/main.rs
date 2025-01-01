use axum::routing::{self};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
