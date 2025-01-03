pub mod domain;
pub mod config;
pub mod app;
pub mod infra;
pub mod presentation;

use std::sync::Arc;

use axum::routing;
use presentation::controllers::users::UsersController;

#[tokio::main]
async fn main() {
    let users_controller = Arc::new(UsersController::new());

    let app = axum::Router::new()
        .route("/users", routing::get(UsersController::get_all))
        .route("/users/{id}", routing::get(UsersController::get_by_id))
        .route("/users", routing::post(UsersController::create))
        .route("/users/{id}", routing::put(UsersController::update))
        .with_state(users_controller);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
