use std::sync::Arc;

use shiori_core::App;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::routes::api;

#[derive(OpenApi)]
#[openapi(info(title = "shiori", description = "TODO DESCRIPTION OF SHIORI",))]
pub struct BaseOpenApi;

impl BaseOpenApi {
    pub fn router<S>() -> OpenApiRouter<S>
    where
        S: Send + Sync + Clone + 'static,
    {
        OpenApiRouter::with_openapi(Self::openapi())
    }

    pub fn build() -> (axum::Router<Arc<App>>, utoipa::openapi::OpenApi) {
        Self::router().merge(api::mount()).split_for_parts()
    }
}
