use axum::Router;

use crate::config::state::AppState;

pub fn mount() -> Router<AppState> {
    Router::new()
}
