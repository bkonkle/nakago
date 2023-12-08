use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

/// A wrapper to implement IntoResponse for Nakago injection errors
pub struct Error(pub nakago::inject::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
        }
    }
}
