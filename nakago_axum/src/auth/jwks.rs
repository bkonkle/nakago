use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use axum::extract::FromRef;
use biscuit::{
    jwk::{AlgorithmParameters, JWKSet, JWK},
    jws::Secret,
    Empty,
};
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use nakago::{Config, Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;
use thiserror::Error;

use super::config::AuthConfig;

/// The JWKS Tag
pub const JWKS: Tag<JWKSet<Empty>> = Tag::new("JWKS");

/// Possible errors during jwks retrieval
#[derive(Debug, Error)]
pub enum JwksClientError {
    /// No key found with the given key_id
    #[error("No key found with the given key_id")]
    MissingKeyId,

    /// Unable to construct RSA public key secret
    #[error("Unable to construct RSA public key secret")]
    SecretKeyError,
}

/// A struct that can retrieve `JWKSet` from a configured Auth url
pub struct JwksClient {
    config: AuthConfig,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl JwksClient {
    /// Create a new instance of the `JwksClient` with the given config Arc reference
    pub fn new(config: AuthConfig) -> Self {
        JwksClient {
            client: Client::builder().build::<_, Body>(HttpsConnector::new()),
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

/// Get a particular key from a key set by id
pub fn get_key(jwks: &JWKSet<Empty>, key_id: &str) -> Result<JWK<Empty>, JwksClientError> {
    let key = jwks
        .find(key_id)
        .ok_or(JwksClientError::MissingKeyId)?
        .clone();

    Ok(key)
}

/// Convert a JWK into a Secret
pub fn get_secret(jwk: JWK<Empty>) -> Result<Secret, JwksClientError> {
    let secret = match jwk.algorithm {
        AlgorithmParameters::RSA(rsa_key) => rsa_key.jws_public_key_secret(),
        _ => return Err(JwksClientError::SecretKeyError),
    };

    Ok(secret)
}

/// A convenience function to get a particular key from a key set, and convert it into a secret
pub fn get_secret_from_key_set(
    jwks: &JWKSet<Empty>,
    key_id: &str,
) -> Result<Secret, JwksClientError> {
    let jwk = get_key(jwks, key_id)?;
    let secret = get_secret(jwk)?;

    Ok(secret)
}

/// Get the default set of JWKS keys
pub async fn init(config: AuthConfig) -> JWKSet<Empty> {
    let jwks_client = JwksClient::new(config);

    jwks_client
        .get_key_set()
        .await
        .expect("Unable to retrieve JWKS")
}

/// Provide the Json Web Key Set
///
/// **Provides:** `Arc<jwks::JWKS>`
///
/// **Depends on:**
///   - `<C: Config>` - requires that `C` fulfills the `AuthConfig: FromRef<C>` constraint
#[derive(Default)]
pub struct ProvideJwks<C: Config> {
    _phantom: PhantomData<C>,
}

#[Provider]
#[async_trait]
impl<C: Config> Provider<JWKSet<Empty>> for ProvideJwks<C>
where
    AuthConfig: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<JWKSet<Empty>>> {
        let config = i.get_type::<C>().await?;
        let auth = AuthConfig::from_ref(&*config);
        let key_set = init(auth).await;

        Ok(Arc::new(key_set))
    }
}