use axum::Router;
use utoipa_redoc::{Redoc, Servable};

use crate::{config::state::AppState, routes::openapi::BaseOpenApi};

mod api;
mod koreader;
mod opds;
mod openapi;

pub fn build_axum_router(state: AppState) -> Router<()> {
    let (router, openapi) = BaseOpenApi::router().merge(api::mount()).split_for_parts();

    router
        .merge(Redoc::with_url("/docs", openapi))
        .with_state(state)
}
