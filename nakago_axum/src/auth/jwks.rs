use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use axum::extract::FromRef;
use biscuit::{
    jwk::{AlgorithmParameters, JWKSet, JWK},
    jws::Secret,
    Empty,
};
use hyper::{self, body::to_bytes, client::HttpConnector, Body, Method, Request};
use hyper_tls::HttpsConnector;
use nakago::{self, inject, Inject, Provider, Tag};
use nakago_derive::Provider;
use thiserror::Error;

use super::Config;

/// The JWKS Tag
pub const JWKS: Tag<JWKSet<Empty>> = Tag::new("auth::JWKS");

/// Get the default set of JWKS keys
pub async fn init(config: Config) -> JWKSet<Empty> {
    let jwks_client = Client::new(config);

    jwks_client
        .get_key_set()
        .await
        .expect("Unable to retrieve JWKS")
}

/// A struct that can retrieve `JWKSet` from a configured Auth url
pub struct Client {
    config: Config,
    client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    /// Create a new instance of the `Client` with the given config Arc reference
    pub fn new(config: Config) -> Self {
        Client {
            client: hyper::Client::builder().build::<_, Body>(HttpsConnector::new()),
            config,
        }
    }

    /// Get a `JWKSet` from the configured Auth url
    pub async fn get_key_set(&self) -> anyhow::Result<JWKSet<Empty>> {
        let url = format!("{}/.well-known/jwks.json", &self.config.url);

        debug!("Fetching keys from '{}'", url);

        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(Body::empty())?;

        let response = self.client.request(req).await?;
        let body = to_bytes(response.into_body()).await?;
        let jwks = serde_json::from_slice::<JWKSet<Empty>>(&body)?;

        Ok(jwks)
    }
}

/// A convenience function to get a particular key from a key set, and convert it into a secret
pub fn get_secret_from_key_set(jwks: &JWKSet<Empty>, key_id: &str) -> Result<Secret, ClientError> {
    let jwk = get_key(jwks, key_id)?;
    let secret = get_secret(jwk)?;

    Ok(secret)
}

/// Get a particular key from a key set by id
pub fn get_key(jwks: &JWKSet<Empty>, key_id: &str) -> Result<JWK<Empty>, ClientError> {
    let key = jwks.find(key_id).ok_or(ClientError::MissingKeyId)?.clone();

    Ok(key)
}

/// Convert a JWK into a Secret
pub fn get_secret(jwk: JWK<Empty>) -> Result<Secret, ClientError> {
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
///
/// **Provides:** `Arc<JWKSet<Empty>>`
///
/// **Depends on:**
///   - `<Config>` - requires that `C` fulfills the `Config: FromRef<C>` constraint
#[derive(Default)]
pub struct Provide<C: nakago::Config> {
    config_tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<C>,
}

impl<C: nakago::Config> Provide<C> {
    /// Create a new instance of Provide
    pub fn new(config_tag: Option<&'static Tag<C>>) -> Self {
        Self {
            config_tag,
            ..Default::default()
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
impl<C: nakago::Config> Provider<JWKSet<Empty>> for Provide<C>
where
    Config: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<JWKSet<Empty>>> {
        let config = if let Some(tag) = self.config_tag {
            i.get(tag).await?
        } else {
            i.get_type::<C>().await?
        };

        let auth = Config::from_ref(&*config);
        let key_set = init(auth).await;

        Ok(Arc::new(key_set))
    }
}
