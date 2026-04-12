use std::path;

use axum::{Json, extract::State, http::StatusCode, middleware};
use serde::Deserialize;
use shiori_api_types::EncodableDirectories;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{AppResult, bad_request, custom},
    middleware::auth::auth_middleware,
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_directories))
        .layer(middleware::from_fn(auth_middleware))
}

/// List filesystem directories.
///
/// The provided path must be **relative** to application's base directory.
#[utoipa::path(
    post,
    path = "/filesystem/directories/list",
    tag = tags::FILESYSTEM,
    security(
        ("cookie" = [])
    ),
    request_body = inline(FolderRequest),
    responses(
        (status = 200, description = "Successfully listed directories", body = inline(EncodableDirectories)),
        (status = 400, description = "Invalid path"),
        (status = 403, description = "Access to the request path is not allowed"),
        (status = 404, description = "Directory does not exist"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_directories(
    State(app): State<AppState>,
    Json(body): Json<FolderRequest>,
) -> AppResult<Json<EncodableDirectories>> {
    let requested_path = path::Path::new(&body.path);

    if requested_path.is_absolute() {
        return Err(bad_request("Absolute paths not allowed"));
    }

    let path = app.base_path.join(requested_path).canonicalize()?;

    if !path.starts_with(&app.base_path) {
        return Err(custom(
            StatusCode::FORBIDDEN,
            "Access to the requested path is not allowed",
        ));
    }

    if !path.is_dir() {
        return Err(bad_request("Path must be a directory"));
    }

    let dirs = shiori_filesystem::common::list_directories(&path, &app.base_path)?;

    let res = EncodableDirectories {
        parent: path
            .parent()
            .and_then(|p| p.strip_prefix(&app.base_path).ok())
            .map(|p| p.to_string_lossy().to_string()),
        directories: dirs,
    };

    Ok(Json(res))
}

#[derive(Deserialize, utoipa::ToSchema)]
struct FolderRequest {
    /// Path of the directory to list its subdirectories.
    #[schema(examples(""))]
    path: String,
}
