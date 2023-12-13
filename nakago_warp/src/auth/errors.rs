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

impl warp::reject::Reject for Error {}
