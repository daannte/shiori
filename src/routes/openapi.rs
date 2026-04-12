use std::sync::Arc;

use shiori_core::App;
use utoipa::{
    Modify, OpenApi,
    openapi::{
        self,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
};
use utoipa_axum::router::OpenApiRouter;

use crate::routes::api;

#[derive(OpenApi)]
#[openapi(
    info(title = "Shiori", description = "TODO"),
    modifiers(&SecurityAddon),
    tags(
        (name = tags::LIBRARY, description = "Library resources and operations"),
        (name = tags::MEDIA, description = "Media management and processing"),
        (name = tags::METADATA, description = "Metadata and informational resources"),
        (name = tags::AUTH, description = "Authentication and authorization"),
        (name = tags::FILESYSTEM, description = "File storage and filesystem operations"),
        (name = tags::SYSTEM, description = "System state and utility operations")
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
        let components = openapi.components.get_or_insert_default();

        let desc = "HttpOnly authentication cookie used to authorize user requests.";
        let cookie = ApiKey::Cookie(ApiKeyValue::with_description("access_token", desc));
        components.add_security_scheme("cookie", SecurityScheme::ApiKey(cookie));
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
    pub const SYSTEM: &str = "system";
}
