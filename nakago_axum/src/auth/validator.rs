use std::sync::Arc;

use async_trait::async_trait;
use biscuit::{jwa::SignatureAlgorithm, jwk::JWKSet, jws::Header, ClaimsSet, Empty, JWT};
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;

use super::{
    jwks::{get_secret_from_key_set, JWKS},
    Error,
};

/// A validator for JWTs that uses a JWKS key set to validate the token
#[derive(Clone)]
pub enum Validator {
    /// A validator that uses a JWKS key set to validate the token
    KeySet(Arc<JWKSet<Empty>>),

    /// A validator that does not validate the token, used for testing
    Unverified,
}

impl Validator {
    /// Get a validated payload from a JWT string
    pub fn get_payload(&self, jwt: &str) -> Result<ClaimsSet<Empty>, Error> {
        match self {
            Validator::KeySet(jwks) => {
                // First extract without verifying the header to locate the key-id (kid)
                let token = JWT::<Empty, Empty>::new_encoded(jwt);

                let header: Header<Empty> = token.unverified_header().map_err(Error::JWTToken)?;

                let key_id = header.registered.key_id.ok_or(Error::JWKSVerification)?;

                debug!("Fetching signing key for '{:?}'", key_id);

                // Now that we have the key, construct our RSA public key secret
                let secret = get_secret_from_key_set(jwks, &key_id)
                    .map_err(|_err| Error::JWKSVerification)?;

                // Now fully verify and extract the token
                let token = token
                    .into_decoded(&secret, SignatureAlgorithm::RS256)
                    .map_err(Error::JWTToken)?;

                let payload = token.payload().map_err(Error::JWTToken)?;

                debug!(
                    "Successfully verified token with subject: {:?}",
                    payload.registered.subject
                );

                Ok(payload.clone())
            }
            Validator::Unverified => {
                let token = JWT::<Empty, Empty>::new_encoded(jwt);

                let payload = &token.unverified_payload().map_err(Error::JWTToken)?;

                Ok(payload.clone())
            }
        }
    }
}

/// Provide the State needed in order to use the `Subject` extractor in an Axum handler
///
/// **Provides:** `Validator`
///
/// **Depends on:**
///   - `Tag(auth::JWKS)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Validator> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Validator>> {
        let jwks = i.get(&JWKS).await?;

        let validator = Validator::KeySet(jwks);

        Ok(Arc::new(validator))
    }
}

/// Provide the test ***unverified*** AuthState used in testing, which trusts any token given to it
///
/// **WARNING: This is insecure and should only be used in testing**
///
/// **Provides:** `Validator`
#[derive(Default)]
pub struct ProvideUnverified {}

#[Provider]
#[async_trait]
impl Provider<Validator> for ProvideUnverified {
    async fn provide(self: Arc<Self>, _i: Inject) -> provider::Result<Arc<Validator>> {
        let validator = Validator::Unverified;

        Ok(Arc::new(validator))
    }
}
