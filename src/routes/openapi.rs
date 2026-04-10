use std::sync::Arc;

use shiori_core::App;
use utoipa::{
    Modify, OpenApi,
    openapi::{
        self,
        security::{HttpBuilder, SecurityScheme},
    },
};
use utoipa_axum::router::OpenApiRouter;

use crate::routes::api;

#[derive(OpenApi)]
#[openapi(
    info(title = "Shiori", description = "TODO"),
    modifiers(&SecurityAddon),
    tags(
        (name = tags::LIBRARY, description = "Library endpoints"),
        (name = tags::MEDIA, description = "Media endpoints"),
        (name = tags::METADATA, description = "Metadata endpoints"),
        (name = tags::AUTH, description = "Auth endpoints"),
        (name = tags::FILESYSTEM, description = "Filesystem endpoints") 
    )
)]
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

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

/// OpenAPI tags used across the API.
///
/// These tags are used to group endpoints in the generated OpenAPI documentation.
/// They should be reused across all route definitions to ensure stable API grouping.
pub mod tags {
    pub const LIBRARY: &str = "library";
    pub const MEDIA: &str = "media";
    pub const METADATA: &str = "metadata";
    pub const AUTH: &str = "auth";
    pub const FILESYSTEM: &str = "filesystem";
}
