use axum::{Json, extract::Query, middleware};
use serde::Deserialize;
use shiori_api_types::EncodableMetadataSearch;
use shiori_metadata::{
    GoodreadsProvider,
    provider::{BooksParams, MetadataProvider},
};
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{AppResult, bad_request},
    middleware::auth::auth_middleware,
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(search_books))
        .routes(routes!(get_book_metadata))
        .layer(middleware::from_fn(auth_middleware))
}

/// Search for book metadata.
#[utoipa::path(
    get,
    path = "/metadata/book",
    params(ListQueryParams),
    tag = tags::METADATA,
    security(
        ("cookie" = [])
    ),
    responses(
        (status = 200, description = "Successfully found book metadata", body = inline(EncodableMetadataSearch)),
        (status = 400, description = "Invalid query parameters"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_book_metadata(
    Query(params): Query<ListQueryParams>,
) -> AppResult<Json<EncodableMetadataSearch>> {
    let metadata = match params.provider.as_str() {
        "goodreads" => GoodreadsProvider::search_id(&params.q_string).await?,
        _ => return Err(bad_request("Unknown metadata provider")),
    };
    Ok(Json(metadata))
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct ListQueryParams {
    /// A search query string.
    #[serde(rename = "q")]
    q_string: String,

    /// The provider to use for the search.
    /// Defaults to "goodreads" if not provided in the query.
    #[serde(default = "default_provider")]
    provider: String,
}

/// Search for books.
#[utoipa::path(
    get,
    path = "/metadata/search",
    params(BookQueryParams),
    tag = tags::METADATA,
    security(
        ("cookie" = [])
    ),
    responses(
        (status = 200, description = "Successfully found books", body=inline(Vec<EncodableMetadataSearch>)),
        (status = 400, description = "Invalid query parameters"),
        (status = 500, description = "Internal server error")
    )
)]
async fn search_books(
    Query(params): Query<BookQueryParams>,
) -> AppResult<Json<Vec<EncodableMetadataSearch>>> {
    let books = match params.provider.as_str() {
        "goodreads" => GoodreadsProvider::search_books(params.into()).await?,
        _ => return Err(bad_request("Unknown provider")),
    };
    Ok(Json(books))
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct BookQueryParams {
    /// The name of the author to search for.
    author: String,

    /// The title of the book to search for.
    title: String,

    /// The provider to use for the search.
    /// Defaults to "goodreads" if not provided in the query.
    #[serde(default = "default_provider")]
    provider: String,
}

impl From<BookQueryParams> for BooksParams {
    fn from(value: BookQueryParams) -> Self {
        Self {
            author: value.author,
            title: value.title,
        }
    }
}

fn default_provider() -> String {
    "goodreads".to_string()
}
