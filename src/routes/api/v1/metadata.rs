use axum::{Json, extract::Query};
use serde::Deserialize;
use shiori_api_types::EncodableMetadataSearch;
use shiori_metadata::{GoodreadsProvider, provider::MetadataProvider};
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{AppResult, bad_request},
    routes::openapi::tags,
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(search_books))
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct Params {
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
    params(Params),
    tag = tags::METADATA,
    security(
        ("cookie" = []),
        ("api_token" = [])
    ),
    responses(
        (status = 200, description = "Successfully found books", body=inline(Vec<EncodableMetadataSearch>)),
        (status = 400, description = "Invalid query parameters"),
        (status = 500, description = "Internal server error")
    )
)]
async fn search_books(
    Query(params): Query<Params>,
) -> AppResult<Json<Vec<EncodableMetadataSearch>>> {
    let books = match params.provider.as_str() {
        "goodreads" => GoodreadsProvider::search_books(params.q_string).await?,
        _ => return Err(bad_request("Unknown provider")),
    };
    Ok(Json(books))
}

fn default_provider() -> String {
    "goodreads".to_string()
}
