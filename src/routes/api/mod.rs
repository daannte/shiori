use utoipa_axum::router::OpenApiRouter;

use crate::config::state::AppState;

pub mod v1;

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/api", OpenApiRouter::new().nest("/v1", v1::mount()))
}

pub fn mount_public() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/api", OpenApiRouter::new().nest("/v1", v1::mount_public()))
}
