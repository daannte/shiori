use utoipa_axum::router::OpenApiRouter;

pub mod library;
pub mod metadata;

use crate::config::state::AppState;

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .merge(metadata::mount())
        .merge(library::mount())
}
