use utoipa_axum::router::OpenApiRouter;

use crate::config::state::AppState;

pub mod sync;

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/koreader", sync::mount())
}
