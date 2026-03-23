use std::fs;
use std::path;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use shiori_api_types::EncodableLibrary;
use shiori_database::models::Library;
use shiori_database::models::NewLibrary;
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
        (status = 200, description = "Successfully fetched libraries", body = Vec<EncodableLibrary>),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_libraries(State(app): State<AppState>) -> APIResult<Json<Vec<EncodableLibrary>>> {
    let mut conn = app.db().await?;

    let libraries = Library::list_libraries(&mut conn)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<EncodableLibrary>>();

    Ok(Json(libraries))
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct NewLibraryRequest {
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
    request_body = NewLibraryRequest,
    responses(
        (status = 200, description = "Successfully created library", body = EncodableLibrary),
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

    let new_library = NewLibrary {
        name: &body.name,
        path: &body.path,
    };

    // TODO: Make sure the library isnt inside another library
    let library = new_library.insert(&mut conn).await?;

    // TODO: Make this atomic with the db insert? (if possible)
    fs::create_dir_all(body.path).map_err(|_| {
        APIError::InternalServerError("Could not create library directory".to_string())
    })?;

    Ok(Json(library.into()))
}

/// Fetch library by id.
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
        (status = 200, description = "Successfully fetched library media"),
        (status = 404, description = "Library not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_library_media(
    Path(_library_id): Path<i32>,
    State(_app): State<AppState>,
) -> APIResult<()> {
    Err(APIError::NotImplemented)
}

/// Upload a new media item to the specified library.
#[utoipa::path(
    post,
    path = "/libraries/{id}/media",
    tag = "library",
    params(
        ("id" = i32, Path, description = "Id of the library")
    ),
    responses(
        (status = 200, description = "Successfully added media to the library"),
        (status = 404, description = "Library not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_library_media(
    Path(_library_id): Path<i32>,
    State(_app): State<AppState>,
) -> APIResult<()> {
    Err(APIError::NotImplemented)
}
