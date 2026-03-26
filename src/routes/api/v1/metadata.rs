use axum::{Json, extract::Query};
use serde::Deserialize;
use shiori_api_types::EncodableMetadataSearch;
use shiori_metadata::{GoodreadsProvider, MetadataProvider};
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::state::AppState,
    errors::{APIError, APIResult},
};

pub fn mount() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(search_metadata))
}

/// Search for metadata.
#[utoipa::path(
    get,
    path = "/metadata/search",
    params(ListQueryParams),
    tag = "metadata",
    responses(
        (status = 200, description = "Successfully found metadata", body = inline(EncodableMetadataSearch)),
        (status = 400, description = "Invalid query parameters"),
        (status = 500, description = "Internal server error")
    )
)]
async fn search_metadata(
    Query(params): Query<ListQueryParams>,
) -> APIResult<Json<EncodableMetadataSearch>> {
    let metadata = match params.provider.as_str() {
        "goodreads" => GoodreadsProvider::search(&params.q_string).await?,
        _ => return Err(APIError::BadRequest("Unknown provider".to_string())),
    };
    Ok(Json(metadata))
}

#[derive(Debug, Deserialize, IntoParams)]
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

fn default_provider() -> String {
    "goodreads".to_string()
}
