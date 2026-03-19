use axum::Router;

use crate::config::state::AppState;

pub mod v1;

pub fn mount() -> Router<AppState> {
    Router::new().nest("/api", Router::new().nest("v1/", v1::mount()))
}
