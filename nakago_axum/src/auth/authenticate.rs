#![allow(unused_imports)]
use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, State},
    Extension,
};
use biscuit::{
    jwa::SignatureAlgorithm,
    jwk::JWKSet,
    jws::{Compact, Header},
    ClaimsSet, Empty, JWT,
};
use http::{header::AUTHORIZATION, request::Parts, HeaderMap, HeaderValue};
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;

use super::{
    errors::AuthError::{self, InvalidAuthHeaderError},
    jwks::{get_secret_from_key_set, JWKSValidator, JWKS},
};

/// The AuthState Tag
pub const AUTH_STATE: Tag<AuthState> = Tag::new("AuthState");

const BEARER: &str = "Bearer ";

/// The state interface needed for Authentication
#[derive(Clone)]
#[allow(dead_code)]
pub struct AuthState {
    /// The JWKS Validator
    pub jwks: JWKSValidator,
}

impl AuthState {
    /// Create a new AuthState instance
    pub(crate) fn new(jwks: JWKSValidator) -> Self {
        Self { jwks }
    }
}

/// The token's Subject claim, which corresponds with the username
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subject(pub Option<String>);

/// Implement the Axum FromRequestParts trait, allowing the `Subject` to be used as an Axum
/// extractor.
#[async_trait]
impl<S> FromRequestParts<S> for Subject
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let state = AuthState::from_ref(state);

        match jwt_from_header(&parts.headers) {
            Ok(Some(jwt)) => {
                let payload = state.jwks.get_payload(jwt)?;
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
pub fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<Option<&str>, AuthError> {
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
        return Err(InvalidAuthHeaderError);
    }

    Ok(Some(auth_header.trim_start_matches(BEARER)))
}

/// Provide the AuthState needed in order to use the `Subject` extractor in an Axum handler
///
/// **Provides:** `AuthState`
///
/// **Depends on:**
///   - `Tag(JWKS)`
#[derive(Default)]
pub struct ProvideAuthState {}

#[Provider]
#[async_trait]
impl Provider<AuthState> for ProvideAuthState {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<AuthState>> {
        let jwks = i.get(&JWKS).await?;
        let auth_state = AuthState::new(JWKSValidator::KeySet(jwks));

        Ok(Arc::new(auth_state))
    }
}

/// Provide the test ***unverified*** AuthState used in testing, which trusts any token given to it
///
/// **WARNING: This is insecure and should only be used in testing**
///
/// **Provides:** `AuthState`
#[derive(Default)]
pub struct ProvideUnverifiedAuthState {}

#[Provider]
#[async_trait]
impl Provider<AuthState> for ProvideUnverifiedAuthState {
    async fn provide(self: Arc<Self>, _i: Inject) -> InjectResult<Arc<AuthState>> {
        let auth_state = AuthState::new(JWKSValidator::Unverified);

        Ok(Arc::new(auth_state))
    }
}
