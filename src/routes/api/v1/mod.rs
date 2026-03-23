use utoipa_axum::router::OpenApiRouter;

pub mod library;
// pub mod media;
pub mod metadata;

use crate::config::state::AppState;

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        // .merge(media::mount())
        .merge(metadata::mount())
        .merge(library::mount())
}
