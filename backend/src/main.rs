use axum::Router;

use crate::config::state::AppState;

mod config;
mod db;
mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .merge(routes::mount())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
