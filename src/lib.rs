pub use routes::openapi::BaseOpenApi;

mod config;
pub mod errors;
mod routes;

use shiori_core::App;
use std::sync::Arc;

use crate::routes::build_axum_router;

pub fn build_handler(app: Arc<App>) -> axum::Router {
    build_axum_router(app)
}
