use axum::{Router, extract::DefaultBodyLimit};
use utoipa_redoc::{Redoc, Servable};

use crate::{config::state::AppState, routes::openapi::BaseOpenApi};

mod api;
mod koreader;
mod opds;
pub mod openapi;

pub fn build_axum_router(state: AppState) -> Router<()> {
    let (router, openapi) = BaseOpenApi::build(state.clone());

    router
        .merge(Redoc::with_url("/docs", openapi))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024)) // 1GiB
        .with_state(state)
}
