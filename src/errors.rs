// https://github.com/rust-lang/crates.io/blob/main/src/util/errors.rs

use std::{borrow::Cow, error::Error, fmt};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::json;
use diesel::result::Error as DieselError;
use shiori_filesystem::FilesystemError;
use shiori_metadata::MetadataError;

pub type BoxedAppError = Box<dyn AppError>;

pub fn bad_request<S: ToString>(error: S) -> BoxedAppError {
    custom(StatusCode::BAD_REQUEST, error.to_string())
}

pub fn server_error<S: ToString>(error: S) -> BoxedAppError {
    custom(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

pub fn unauthorized<S: ToString>(error: S) -> BoxedAppError {
    custom(StatusCode::UNAUTHORIZED, error.to_string())
}

pub fn not_found() -> BoxedAppError {
    custom(StatusCode::NOT_FOUND, "Not Found")
}

pub fn service_unavailable() -> BoxedAppError {
    custom(StatusCode::SERVICE_UNAVAILABLE, "Service unavailable")
}

// =============================================================================
// AppError trait

pub trait AppError: Send + fmt::Display + fmt::Debug + 'static {
    fn response(&self) -> axum::response::Response;
}

impl AppError for BoxedAppError {
    fn response(&self) -> axum::response::Response {
        (**self).response()
    }
}

impl IntoResponse for BoxedAppError {
    fn into_response(self) -> Response {
        self.response()
    }
}

pub type AppResult<T> = Result<T, BoxedAppError>;

// =============================================================================
// JSON custom message

pub fn custom(status: StatusCode, detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    Box::new(CustomApiError {
        status,
        detail: detail.into(),
    })
}

fn json_error(status: StatusCode, detail: &str) -> Response {
    let json = json!({
        "error": detail,
    });
    (status, json).into_response()
}

#[derive(Clone, Debug)]
pub struct CustomApiError {
    status: StatusCode,
    detail: Cow<'static, str>,
}

impl fmt::Display for CustomApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail.fmt(f)
    }
}

impl AppError for CustomApiError {
    fn response(&self) -> axum::response::Response {
        json_error(self.status, &self.detail)
    }
}

// =============================================================================
// Error impls

impl<E: Error + Send + 'static> AppError for E {
    fn response(&self) -> axum::response::Response {
        tracing::error!(error = %self, "Internal server error");

        server_error("Internal server error").into_response()
    }
}

impl From<DieselError> for BoxedAppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => not_found(),
            _ => Box::new(err),
        }
    }
}

impl From<diesel_async::pooled_connection::deadpool::PoolError> for BoxedAppError {
    fn from(err: diesel_async::pooled_connection::deadpool::PoolError) -> BoxedAppError {
        tracing::error!("Database pool error: {err}");
        service_unavailable()
    }
}

impl From<serde_json::Error> for BoxedAppError {
    fn from(err: serde_json::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<std::io::Error> for BoxedAppError {
    fn from(err: std::io::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<FilesystemError> for BoxedAppError {
    fn from(err: FilesystemError) -> Self {
        server_error(err)
    }
}

impl From<MetadataError> for BoxedAppError {
    fn from(err: MetadataError) -> Self {
        match err {
            MetadataError::Network(e) => server_error(format!(
                "Network error while getting book information: {}",
                e
            )),
            MetadataError::MissingTag(tag) => server_error(format!(
                "Missing expected tag during book information retrieval: {}",
                tag
            )),
            MetadataError::JsonParse(e) => server_error(format!(
                "Failed to parse JSON while retrieving book information: {}",
                e
            )),
            MetadataError::MissingBookInfo => {
                server_error("No book information found in the provider response")
            }
            MetadataError::Other(msg) => server_error(format!("Unexpected error: {}", msg)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_error_responses() {
        use serde::de::Error;

        assert_eq!(bad_request("").response().status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            server_error("").response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            unauthorized("").response().status(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            BoxedAppError::from(DieselError::NotFound)
                .response()
                .status(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            server_error("").response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );

        assert_eq!(
            BoxedAppError::from(serde_json::Error::custom("ExpectedColon"))
                .response()
                .status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
