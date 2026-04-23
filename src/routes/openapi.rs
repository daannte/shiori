use std::sync::Arc;

use axum::{http::header, middleware};
use shiori_core::App;
use utoipa::{
    Modify, OpenApi,
    openapi::{
        self,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    config::state::AppState,
    middleware::auth::{auth_middleware, url_auth_middleware},
    routes::{api, koreader},
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Shiori", description = "TODO"),
    modifiers(&SecurityAddon),
    tags(
        (name = tags::LIBRARY, description = "Library resources"),
        (name = tags::MEDIA, description = "Media management"),
        (name = tags::METADATA, description = "Metadata resouces"),
        (name = tags::AUTH, description = "Authentication and authorization"),
        (name = tags::TOKENS, description = "Management of API tokens"),
        (name = tags::FILESYSTEM, description = "Filesystem operations"),
        (name = tags::SYSTEM, description = "System state"),
        (name = tags::KOREADER, description = "KOReader synchronization")
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

    pub fn build(state: AppState) -> (axum::Router<Arc<App>>, utoipa::openapi::OpenApi) {
        let public = Self::router().merge(api::mount_public());

        let private =
            OpenApiRouter::new()
                .merge(api::mount())
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ));

        // TODO: Add OPDS here
        let special = OpenApiRouter::new()
            .merge(koreader::mount())
            .layer(middleware::from_fn_with_state(state, url_auth_middleware));

        public.merge(private).merge(special).split_for_parts()
    }
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.get_or_insert_default();

        let desc = "The authentication cookie used to authorize user requests.";
        let cookie = ApiKey::Cookie(ApiKeyValue::with_description("access_token", desc));
        components.add_security_scheme("cookie", SecurityScheme::ApiKey(cookie));

        let desc = "The API token used to authenticate requests.";
        let api_token = ApiKey::Header(ApiKeyValue::with_description(
            header::AUTHORIZATION.as_str(),
            desc,
        ));
        components.add_security_scheme("api_token", SecurityScheme::ApiKey(api_token));
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
    pub const TOKENS: &str = "tokens";
    pub const KOREADER: &str = "koreader";
}
