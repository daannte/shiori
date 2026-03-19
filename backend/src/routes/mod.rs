use axum::Router;

use crate::config::state::AppState;

mod api;
mod koreader;
mod opds;

pub fn mount() -> Router<AppState> {
    let router = Router::new();

    router.merge(api::mount())
}
