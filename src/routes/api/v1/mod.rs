use utoipa_axum::router::OpenApiRouter;

pub mod auth;
pub mod filesystem;
pub mod library;
pub mod media;
pub mod metadata;

use crate::config::state::AppState;

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .merge(filesystem::mount())
        .merge(library::mount())
        .merge(media::mount())
        .merge(metadata::mount())
        .merge(auth::mount())
}
