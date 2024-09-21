use std::any::Any;

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderValue},
};
use biscuit::{ClaimsSet, Empty};
use hyper::{header::AUTHORIZATION, HeaderMap};
use serde::{Deserialize, Serialize};

use crate::State;

use super::{
    validator::Validator,
    Error::{self, InvalidAuthHeader, MissingValidator},
    JWKSValidator,
};

const BEARER: &str = "Bearer ";

/// The Token, including the JWT string and the payload with claims
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Token<T: Default = Empty> {
    /// The JWT string itself
    pub jwt: Option<String>,

    /// The decoded JWT claims
    pub claims: Option<ClaimsSet<T>>,
}

/// Implement the Axum FromRequestParts trait, allowing the `Subject` to be used as an Axum
/// extractor.
#[async_trait]
impl<T> FromRequestParts<State> for Token<T>
where
    T: Default + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + Any,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &State,
    ) -> std::result::Result<Self, Self::Rejection> {
        let validator = state
            .get::<JWKSValidator<T>>()
            .await
            .map_err(|_err| MissingValidator)?;

        match jwt_from_header(&parts.headers) {
            Ok(Some(jwt)) => {
                let payload = validator.get_payload(jwt)?;

                Ok(Token {
                    jwt: Some(jwt.to_string()),
                    claims: Some(payload),
                })
            }
            Ok(None) => Ok(Token::<T>::default()),
            Err(e) => Err(e),
        }
    }
}

/// The token's Subject claim, which corresponds with the username
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subject(pub Option<String>);

/// Implement the Axum FromRequestParts trait, allowing the `Subject` to be used as an Axum
/// extractor.
#[async_trait]
impl FromRequestParts<State> for Subject {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &State,
    ) -> std::result::Result<Self, Self::Rejection> {
        let validator = state
            .get::<JWKSValidator>()
            .await
            .map_err(|_err| MissingValidator)?;

        match jwt_from_header(&parts.headers) {
            Ok(Some(jwt)) => {
                let payload: ClaimsSet<Empty> = validator.get_payload(jwt)?;
                let subject = payload.registered.subject.clone();

                debug!("Successfully verified token with subject: {:?}", subject);

                Ok(Subject(subject))
            }
            Ok(None) => Ok(Subject(None)),
            Err(e) => Err(e),
        }
    }
}

/// If an authorization header is provided, make sure it's in the expected format, and
/// return it as a String.
fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<Option<&str>, Error> {
    let header = if let Some(value) = headers.get(AUTHORIZATION) {
        value
    } else {
        // No Authorization header found, so return early with None
        return Ok(None);
    };

    let auth_header = if let Ok(value) = std::str::from_utf8(header.as_bytes()) {
        value
    } else {
        // Authorization header couldn't be decoded, so return early with None
        return Ok(None);
    };

    if !auth_header.starts_with(BEARER) {
        // Authorization header doesn't start with "Bearer ", so return early with an Error
        return Err(InvalidAuthHeader);
    }

    Ok(Some(auth_header.trim_start_matches(BEARER)))
}
