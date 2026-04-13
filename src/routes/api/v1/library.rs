use shiori_filesystem::media::epub::Epub;
use std::ffi::OsStr;
use std::path;
use tokio::fs;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    middleware,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::Deserialize;
use shiori_api_types::{EncodableLibrary, EncodableMedia};
use shiori_database::models::{Library, Media, NewLibrary, NewMedia, PatchMedia};
use tempfile::NamedTempFile;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{AppResult, bad_request, custom},
    middleware::auth::auth_middleware,
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_libraries, create_library))
        .routes(routes!(list_library_media, create_library_media))
        .layer(middleware::from_fn(auth_middleware))
}

/// List all libraries.
#[utoipa::path(
    get,
    path = "/libraries",
    tag = tags::LIBRARY,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully fetched libraries", body = inline(Vec<EncodableLibrary>)),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_libraries(State(app): State<AppState>) -> AppResult<Json<Vec<EncodableLibrary>>> {
    let mut conn = app.db().await?;

    let libraries = Library::all(&mut conn)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<EncodableLibrary>>();

    Ok(Json(libraries))
}

#[derive(Deserialize, utoipa::ToSchema)]
struct NewLibraryRequest {
    /// Name of the library.
    #[schema(examples("Light Novels"))]
    pub name: String,
    /// File system path to the library's directory, relative to the application's base directory.
    #[schema(examples("light_novels"))]
    pub path: String,
}

/// Create a new library.
///
/// The directory must already exist on the system.
#[utoipa::path(
    post,
    path = "/libraries",
    tag = tags::LIBRARY,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    request_body = inline(NewLibraryRequest),
    responses(
        (status = 200, description = "Successfully created library", body = inline(EncodableLibrary)),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_library(
    State(app): State<AppState>,
    Json(body): Json<NewLibraryRequest>,
) -> AppResult<Json<EncodableLibrary>> {
    if path::Path::new(&body.path).is_absolute() {
        return Err(bad_request("Absolute paths not allowed"));
    }

    let new_path = app.base_path.join(body.path).canonicalize()?;

    if !new_path.starts_with(&app.base_path) {
        return Err(custom(
            StatusCode::FORBIDDEN,
            "Access to the requested path is not allowed",
        ));
    }

    if !new_path.is_dir() {
        return Err(bad_request("Path must be a directory"));
    }

    let mut conn = app.db().await?;

    // NOTE: There has to be a better way to do this
    let libraries = Library::all(&mut conn).await?;

    for lib in libraries {
        let lib_path = path::Path::new(&lib.path);

        if new_path.starts_with(lib_path) {
            return Err(bad_request(format!(
                "Library path '{}' is inside existing library '{}'",
                new_path.display(),
                lib.path
            )));
        }

        if lib_path.starts_with(&new_path) {
            return Err(bad_request(format!(
                "Library path '{}' contains existing library '{}'",
                new_path.display(),
                lib.path
            )));
        }
    }

    let path_str = new_path
        .to_str()
        .ok_or_else(|| bad_request("Path contains invalid UTF-8"))?;

    let new_library = NewLibrary {
        name: &body.name,
        path: path_str,
    };

    tracing::debug!(?new_library, "Inserting new library into DB");

    let library = new_library.insert(&conn).await?;

    Ok(Json(library.into()))
}

/// List library media.
#[utoipa::path(
    get,
    path = "/libraries/{id}/media",
    tag = tags::LIBRARY,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    params(
        ("id" = i32, Path, description = "Id of the library")
    ),
    responses(
        (status = 200, description = "Successfully fetched library media", body = inline(Vec<EncodableMedia>)),
        (status = 404, description = "Library not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_library_media(
    Path(library_id): Path<i32>,
    State(app): State<AppState>,
) -> AppResult<Json<Vec<EncodableMedia>>> {
    let mut conn = app.db().await?;

    let media = Media::find_by_library_id(&mut conn, library_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();

    Ok(Json(media))
}

#[derive(TryFromMultipart, ToSchema)]
struct NewMediaRequest {
    /// An array of files to upload.
    #[schema(format = Binary, value_type = Vec<u8>)]
    files: Vec<FieldData<NamedTempFile>>,
}

/// Upload a new media item to the specified library.
#[utoipa::path(
    post,
    path = "/libraries/{id}/media",
    tag = tags::LIBRARY,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    params(
        ("id" = i32, Path, description = "Id of the library")
    ),
    request_body(
        content = inline(NewMediaRequest),
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully added media to the library", body = inline(Vec<EncodableMedia>)),
        (status = 400, description = "Invalid uploaded media"),
        (status = 404, description = "Library not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_library_media(
    Path(library_id): Path<i32>,
    State(app): State<AppState>,
    TypedMultipart(body): TypedMultipart<NewMediaRequest>,
) -> AppResult<Json<Vec<EncodableMedia>>> {
    // TODO:
    // - Refactor this func
    // - Keep running even if some files fail
    let mut conn = app.db().await?;

    let mut uploaded: Vec<EncodableMedia> = Vec::new();

    let library = Library::find(&mut conn, library_id).await?;

    for f in body.files {
        let file_name = f
            .metadata
            .file_name
            .as_deref()
            .ok_or_else(|| bad_request("Media must have a filename.".to_string()))?;

        let path = path::Path::new(file_name);

        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap();

        let ext = path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_ascii_lowercase)
            .ok_or_else(|| bad_request("Media must have an extension.".to_string()))?;

        let media_path = path::Path::new(&library.path).join(file_name);

        if fs::metadata(&media_path).await.is_ok() {
            return Err(bad_request("Media already exists in library".to_string()));
        }

        let new_media = NewMedia {
            name: file_stem,
            size: f
                .contents
                .as_file()
                .metadata()
                .unwrap()
                .len()
                .try_into()
                .unwrap_or_else(|_| {
                    tracing::error!(?media_path, "Failed to convert file size to i64");
                    0
                }),
            path: &media_path.to_string_lossy(),
            extension: &ext,
            library_id,
            cover_path: None,
        };

        tracing::debug!(?new_media, "Inserting new media into DB");
        let media = new_media.insert(&mut conn).await?;
        fs::copy(f.contents.path(), media_path).await?;

        let cover_path = Epub::get_cover_path(&media.id, f.contents.path(), &app.base_path);

        let patch = PatchMedia {
            cover_path: cover_path.as_deref(),
            ..Default::default()
        };
        let m = patch.update(&mut conn, media).await?;
        tracing::debug!("Updated media with cover path");

        uploaded.push(m.into());
    }

    Ok(Json(uploaded))
}
