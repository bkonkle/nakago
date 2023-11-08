use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use thiserror::Error;

/// Expected Error Cases
#[derive(Error, Debug)]
pub enum Error {
    /// The Authorizat ion header is not valid
    #[error("Invalid Authorization header")]
    InvalidAuthHeader,

    /// An error occurred while attempting to decode the token
    #[error("Invalid JWT")]
    JWTToken(biscuit::errors::Error),

    /// An error occured while attempting to identify the key id
    #[error("JWK verification failed")]
    JWKSVerification,

    /// An error occured while attempting to resolve the Validator dependency
    #[error("Missing Validator dependency")]
    MissingValidator,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::JWKSVerification => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            Error::JWTToken(err) => {
                (StatusCode::BAD_REQUEST, format!("JWTToken Error: {err}")).into_response()
            }
            Error::InvalidAuthHeader => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::MissingValidator => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
