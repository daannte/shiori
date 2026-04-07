use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shiori_metadata::MetadataError;
use thiserror::Error;

pub type APIResult<T> = Result<T, APIError>;

#[allow(unused)]
#[derive(Debug, Error)]
pub enum APIError {
    #[error("{0}")]
    InternalServerError(String),
    #[error("This has not been implemented yet")]
    NotImplemented,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    DbError(#[from] diesel::result::Error),
}

impl APIError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            APIError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            APIError::BadRequest(_) => StatusCode::BAD_REQUEST,
            APIError::Forbidden(_) => StatusCode::FORBIDDEN,
            APIError::NotFound(_) => StatusCode::NOT_FOUND,
            APIError::DbError(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<diesel_async::pooled_connection::deadpool::PoolError> for APIError {
    fn from(error: diesel_async::pooled_connection::deadpool::PoolError) -> Self {
        APIError::InternalServerError(error.to_string())
    }
}

impl From<MetadataError> for APIError {
    fn from(error: MetadataError) -> Self {
        match error {
            MetadataError::Network(_) => {
                APIError::InternalServerError("Network error contacting provider".into())
            }

            MetadataError::HtmlParse => {
                APIError::InternalServerError("Failed to parse HTML from provider".into())
            }

            MetadataError::MissingTag(_) => {
                APIError::InternalServerError("Provider response missing expected tag".into())
            }

            MetadataError::JsonParse(_) => {
                APIError::InternalServerError("Failed to parse JSON from provider".into())
            }

            MetadataError::MissingBookInfo => {
                APIError::InternalServerError("Book info not found in provider data".into())
            }

            MetadataError::Other(msg) => APIError::InternalServerError(msg),
        }
    }
}

impl From<std::io::Error> for APIError {
    fn from(error: std::io::Error) -> APIError {
        APIError::InternalServerError(error.to_string())
    }
}

#[derive(Debug)]
pub struct ApiErrorResponse {
    status: StatusCode,
    message: String,
}

impl From<APIError> for ApiErrorResponse {
    fn from(error: APIError) -> Self {
        ApiErrorResponse {
            status: error.status_code(),
            message: error.to_string(),
        }
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "status": self.status.as_u16(),
            "message": self.message,
        }))
        .into_response();

        let builder = Response::builder()
            .header("Content-Type", "application/json")
            .status(self.status);

        builder.body(body.into_body()).unwrap_or_else(|error| {
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        })
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        ApiErrorResponse::from(self).into_response()
    }
}
