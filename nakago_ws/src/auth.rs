use std::{any::Any, collections::HashMap, marker::PhantomData};

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use biscuit::{ClaimsSet, Empty};
use derive_new::new;
use http::{request::Parts, Uri};
use nakago_axum::{
    auth::{
        self,
        validator::Validator,
        Error::{InvalidAuthHeader, MissingValidator},
    },
    State,
};
use serde::{Deserialize, Serialize};

/// The JWT Token's Registered Claims
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Token<PrivateClaims = Empty> {
    /// The JWT string itself
    pub jwt: Option<String>,

    /// The JWT's Registered Claims
    pub claims: Option<ClaimsSet<PrivateClaims>>,
}

/// Implement the Axum FromRequestParts trait, allowing `Claims` to be used as an Axum extractor.
#[async_trait]
impl<PrivateClaims> FromRequestParts<State> for Token<PrivateClaims>
where
    PrivateClaims: Default + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + Any,
{
    type Rejection = auth::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &State,
    ) -> std::result::Result<Self, Self::Rejection> {
        let validator = state
            .get::<Box<dyn Validator<PrivateClaims>>>()
            .await
            .map_err(|_err| MissingValidator)?;

        match jwt_from_params(&parts.uri) {
            Ok(Some(jwt)) => {
                let payload = validator.get_payload(&jwt)?;

                debug!(
                    "Successfully verified token with subject: {:?}",
                    payload.registered.subject
                );

                Ok(Token {
                    jwt: Some(jwt),
                    claims: Some(payload),
                })
            }
            Ok(None) => Ok(Token::default()),
            Err(e) => Err(e),
        }
    }
}

/// The token's Subject claim, which corresponds with the username
#[derive(Debug, PartialEq, Eq, Clone, new)]
pub struct Subject<PrivateClaims> {
    /// The token subject claim
    pub username: Option<String>,
    _phantom: PhantomData<PrivateClaims>,
}

/// Implement the Axum FromRequestParts trait, allowing the `Subject` to be used as an Axum
/// extractor.
#[async_trait]
impl<PrivateClaims: Any> FromRequestParts<State> for Subject<PrivateClaims> {
    type Rejection = auth::Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &State,
    ) -> std::result::Result<Self, Self::Rejection> {
        let validator = state
            .get::<Box<dyn Validator<PrivateClaims>>>()
            .await
            .map_err(|_err| MissingValidator)?;

        match jwt_from_params(&parts.uri) {
            Ok(Some(jwt)) => {
                let payload: ClaimsSet<PrivateClaims> = validator.get_payload(&jwt)?;
                let subject = payload.registered.subject.clone();

                debug!("Successfully verified token with subject: {:?}", subject);

                Ok(Subject::new(subject))
            }
            Ok(None) => Ok(Subject::new(None)),
            Err(e) => Err(e),
        }
    }
}

/// Extract the JWT from the request parameters
pub fn jwt_from_params(uri: &Uri) -> Result<Option<String>, auth::Error> {
    let query = uri.query().unwrap_or_default();

    let params: HashMap<String, String> =
        serde_urlencoded::from_str(query).map_err(|_err| InvalidAuthHeader)?;

    let param = if let Some(value) = params.get("token") {
        value
    } else {
        // No Authorization header found, so return early with None
        return Ok(None);
    };

    let auth_header = if let Ok(value) = std::str::from_utf8(param.as_bytes()) {
        value
    } else {
        // Authorization header couldn't be decoded, so return early with None
        return Ok(None);
    };

    Ok(Some(auth_header.to_string()))
}
