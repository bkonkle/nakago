use std::{any::Any, marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use biscuit::{
    jwa::SignatureAlgorithm, jwk::JWKSet, ClaimPresenceOptions, ClaimsSet, Empty,
    ValidationOptions, JWT,
};
use derive_new::new;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;
use serde::{Deserialize, Serialize};

use super::Error;

/// A trait for validating JWTs
pub trait Validator<T = Empty>: Send + Sync + Any {
    /// Get a validated payload from a JWT string
    fn get_payload(&self, jwt: &str) -> Result<ClaimsSet<T>, Error>;
}

/// A validator for JWTs that uses a JWKS key set to validate the token
#[derive(Clone)]
pub struct JWKSValidator<T = Empty> {
    key_set: Arc<JWKSet<T>>,
}

impl<T> Validator<T> for JWKSValidator<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + Any,
{
    /// Get a validated payload from a JWT string
    fn get_payload(&self, jwt: &str) -> Result<ClaimsSet<T>, Error> {
        let token = JWT::<T, Empty>::new_encoded(jwt)
            .decode_with_jwks(&self.key_set, Some(SignatureAlgorithm::RS256))
            .map_err(|_err| Error::JWKSVerification)?;

        token
            .validate(ValidationOptions {
                claim_presence_options: ClaimPresenceOptions {
                    expiry: biscuit::Presence::Required,
                    subject: biscuit::Presence::Required,
                    ..Default::default()
                },
                ..Default::default()
            })
            .map_err(|_err| Error::JWKSValidation)?;

        let payload = token.payload().map_err(Error::JWTToken)?;

        debug!(
            "Successfully verified token with subject: {:?}",
            payload.registered.subject
        );

        Ok(payload.clone())
    }
}

/// Provide the State needed in order to use the `Subject` extractor in an Axum handler
#[derive(Default)]
pub struct Provide<T = Empty> {
    _phantom: PhantomData<T>,
}

#[Provider]
#[async_trait]
impl<T> Provider<Box<dyn Validator<T>>> for Provide<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + Any,
{
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Box<dyn Validator<T>>>> {
        let jwks = i.get::<JWKSet<T>>().await?;

        let validator = Box::new(JWKSValidator { key_set: jwks });

        Ok(Arc::new(validator))
    }
}

/// A test-only validator that trusts any token given to it
/// WARNING: Not intended for production use, but exported here for integration tests
#[derive(Default, Clone, Debug, PartialEq, Eq, new)]
pub struct UnverifiedDecoder<T> {
    _phantom: PhantomData<T>,
}

impl<T> Validator<T> for UnverifiedDecoder<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + Any,
{
    /// Get a validated payload from a JWT string
    fn get_payload(&self, jwt: &str) -> Result<ClaimsSet<T>, Error> {
        let token = JWT::<T, Empty>::new_encoded(jwt);

        let payload = &token.unverified_payload().map_err(Error::JWTToken)?;

        Ok(payload.clone())
    }
}

/// Provide the test ***unverified*** Validator, which trusts any token given to it
/// WARNING: Not intended for production use, but exported here for integration tests
#[derive(Default)]
pub struct ProvideUnverified<T = Empty> {
    _phantom: PhantomData<T>,
}

#[Provider]
#[async_trait]
impl<T> Provider<Box<dyn Validator<T>>> for ProvideUnverified<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + Any,
{
    async fn provide(self: Arc<Self>, _i: Inject) -> provider::Result<Arc<Box<dyn Validator<T>>>> {
        let validator = Box::new(UnverifiedDecoder::new());

        Ok(Arc::new(validator))
    }
}
