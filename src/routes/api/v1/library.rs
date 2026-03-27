use shiori_filesystem::media::epub::Epub;
use std::ffi::OsStr;
use std::path;
use tokio::fs;

use axum::{
    Json,
    extract::{Path, State},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::Deserialize;
use shiori_api_types::{EncodableLibrary, EncodableMedia};
use shiori_database::models::{Library, Media, NewLibrary, NewMedia};
use tempfile::NamedTempFile;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_libraries, create_library))
        .routes(routes!(list_library_media, create_library_media))
        .routes(routes!(get_library))
}

/// List all libraries.
#[utoipa::path(
    get,
    path = "/libraries",
    tag = "library",
    responses(
        (status = 200, description = "Successfully fetched libraries", body = inline(Vec<EncodableLibrary>)),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_libraries(State(app): State<AppState>) -> APIResult<Json<Vec<EncodableLibrary>>> {
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
    pub name: String,
    /// File system path to the library's directory.
    pub path: String,
}

/// Create a new library.
#[utoipa::path(
    post,
    path = "/libraries",
    tag = "library",
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
) -> APIResult<Json<EncodableLibrary>> {
    if !path::Path::new(&body.path).is_absolute() {
        return Err(APIError::BadRequest(
            "Library path must be absolute".to_string(),
        ));
    }

    let mut conn = app.db().await?;

    // NOTE: There has to be a better way to do this
    let libraries = Library::all(&mut conn).await?;

    let new_path = path::Path::new(&body.path);

    for lib in libraries {
        let lib_path = path::Path::new(&lib.path);

        if new_path.starts_with(lib_path) {
            return Err(APIError::BadRequest(format!(
                "Library path '{}' is inside existing library '{}'",
                new_path.display(),
                lib.path
            )));
        }

        if lib_path.starts_with(new_path) {
            return Err(APIError::BadRequest(format!(
                "Library path '{}' contains existing library '{}'",
                new_path.display(),
                lib.path
            )));
        }
    }

    let new_library = NewLibrary {
        name: &body.name,
        path: &body.path,
    };

    let library = new_library.insert(&conn).await?;

    // TODO: Make this atomic with the db insert? (if possible)
    fs::create_dir_all(body.path).await?;

    Ok(Json(library.into()))
}

/// Fetch library by id.
///
/// Not implemented
#[utoipa::path(
    get,
    path = "/libraries/{id}",
    tag = "library",
    params(
        ("id" = i32, Path, description = "Id of the library")
    ),
    responses(
        (status = 200, description = "Successfully fetched library"),
        (status = 404, description = "Library not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_library(Path(_library_id): Path<i32>, State(_app): State<AppState>) -> APIResult<()> {
    Err(APIError::NotImplemented)
}

/// List library media.
#[utoipa::path(
    get,
    path = "/libraries/{id}/media",
    tag = "library",
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
) -> APIResult<Json<Vec<EncodableMedia>>> {
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
    tag = "library",
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
) -> APIResult<Json<Vec<EncodableMedia>>> {
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
            .ok_or_else(|| APIError::BadRequest("Media must have a filename.".to_string()))?;

        let path = path::Path::new(file_name);

        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap();

        let ext = path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_ascii_lowercase)
            .ok_or_else(|| APIError::BadRequest("Media must have an extension.".to_string()))?;

        let media_path = path::Path::new(&library.path).join(file_name);

        if fs::metadata(&media_path).await.is_ok() {
            return Err(APIError::BadRequest(
                "Media already exists in library".to_string(),
            ));
        }

        let cover_path = Epub::get_cover_path(file_stem, f.contents.path());

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
                    println!("Failed to convert to i64");
                    0
                }),
            path: &media_path.to_string_lossy(),
            extension: &ext,
            library_id,
            cover_path: cover_path.as_deref(),
        };

        let media = new_media.insert(&mut conn).await?;
        uploaded.push(media.into());

        fs::copy(f.contents.path(), media_path).await?;
    }

    Ok(Json(uploaded))
}
