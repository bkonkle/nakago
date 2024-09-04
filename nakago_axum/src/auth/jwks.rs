use std::{any::Any, marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use biscuit::{
    jwk::{AlgorithmParameters, JWK},
    jws::Secret,
};
use nakago::{self, provider, Inject, Provider, Tag};
use nakago_derive::Provider;
use nakago_figment::FromRef;
use serde::Deserialize;
use thiserror::Error;

pub use biscuit::{jwk::JWKSet, Empty}; // Re-export types from biscuit

use super::Config;

/// Get the default set of JWKS keys
pub async fn init<T: for<'de> Deserialize<'de>>(config: Config) -> JWKSet<T> {
    let jwks_client = Client::new(config);

    jwks_client
        .get_key_set::<T>()
        .await
        .expect("Unable to retrieve JWKS")
}

/// A struct that can retrieve `JWKSet` from a configured Auth url
pub struct Client {
    config: Config,
}

impl Client {
    /// Create a new instance of the `Client` with the given config Arc reference
    pub fn new(config: Config) -> Self {
        Client { config }
    }

    /// Get a `JWKSet` from the configured Auth url
    pub async fn get_key_set<T: for<'de> Deserialize<'de>>(&self) -> anyhow::Result<JWKSet<T>> {
        let response = reqwest::get(format!("{}/.well-known/jwks.json", &self.config.url)).await?;
        let jwks = response.json::<JWKSet<T>>().await?;

        Ok(jwks)
    }
}

/// A convenience function to get a particular key from a key set, and convert it into a secret
pub fn get_secret_from_key_set<T: Clone>(
    jwks: &JWKSet<T>,
    key_id: &str,
) -> Result<Secret, ClientError> {
    let jwk = get_key(jwks, key_id)?;
    let secret = get_secret(jwk)?;

    Ok(secret)
}

/// Get a particular key from a key set by id
pub fn get_key<T: Clone>(jwks: &JWKSet<T>, key_id: &str) -> Result<JWK<T>, ClientError> {
    let key = jwks.find(key_id).ok_or(ClientError::MissingKeyId)?.clone();

    Ok(key)
}

/// Convert a JWK into a Secret
pub fn get_secret<T>(jwk: JWK<T>) -> Result<Secret, ClientError> {
    let secret = match jwk.algorithm {
        AlgorithmParameters::RSA(rsa_key) => rsa_key.jws_public_key_secret(),
        _ => return Err(ClientError::SecretKeyError),
    };

    Ok(secret)
}

/// Possible errors during jwks retrieval
#[derive(Debug, Error)]
pub enum ClientError {
    /// No key found with the given key_id
    #[error("No key found with the given key_id")]
    MissingKeyId,

    /// Unable to construct RSA public key secret
    #[error("Unable to construct RSA public key secret")]
    SecretKeyError,
}

/// Provide the Json Web Key Set
#[derive(Default)]
pub struct Provide<C: nakago_figment::Config, T = Empty> {
    config_tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<T>,
}

impl<C: nakago_figment::Config, T: Send + Sync + Any> Provide<C, T> {
    /// Create a new instance of Provide
    pub fn new(config_tag: Option<&'static Tag<C>>) -> Self {
        Self {
            config_tag,
            _phantom: PhantomData,
        }
    }

    /// Set the config Tag for this instance
    pub fn with_config_tag(self, config_tag: &'static Tag<C>) -> Self {
        Self {
            config_tag: Some(config_tag),
            ..self
        }
    }
}

#[Provider]
#[async_trait]
impl<C: nakago_figment::Config, T: Send + Sync + Any + for<'de> Deserialize<'de>>
    Provider<JWKSet<T>> for Provide<C, T>
where
    Config: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<JWKSet<T>>> {
        let config = if let Some(tag) = self.config_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<C>().await?
        };

        let auth = Config::from_ref(&*config);
        let key_set = init(auth).await;

        Ok(Arc::new(key_set))
    }
}
