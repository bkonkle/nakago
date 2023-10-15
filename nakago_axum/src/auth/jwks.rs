use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use axum::extract::FromRef;
use biscuit::{
    jwa::SignatureAlgorithm,
    jwk::{AlgorithmParameters, JWKSet, JWK},
    jws::{Header, Secret},
    ClaimsSet, Empty, JWT,
};
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use nakago::{Config, Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;
use thiserror::Error;

use super::{config::AuthConfig, AuthError};

/// The JWKS Tag
pub const JWKS: Tag<JWKSet<Empty>> = Tag::new("JWKS");

/// Get the default set of JWKS keys
pub async fn init(config: AuthConfig) -> JWKSet<Empty> {
    let jwks_client = JwksClient::new(config);

    jwks_client
        .get_key_set()
        .await
        .expect("Unable to retrieve JWKS")
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

/// A convenience function to get a particular key from a key set, and convert it into a secret
pub fn get_secret_from_key_set(
    jwks: &JWKSet<Empty>,
    key_id: &str,
) -> Result<Secret, JwksClientError> {
    let jwk = get_key(jwks, key_id)?;
    let secret = get_secret(jwk)?;

    Ok(secret)
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

/// A validator for JWTs that uses a JWKS key set to validate the token
#[derive(Clone)]
pub enum JWKSValidator {
    /// A validator that uses a JWKS key set to validate the token
    KeySet(Arc<JWKSet<Empty>>),

    /// A validator that does not validate the token, used for testing
    Unverified,
}

impl JWKSValidator {
    /// Get a validated payload from a JWT string
    pub fn get_payload(&self, jwt: &str) -> Result<ClaimsSet<Empty>, AuthError> {
        match self {
            JWKSValidator::KeySet(jwks) => {
                // First extract without verifying the header to locate the key-id (kid)
                let token = JWT::<Empty, Empty>::new_encoded(jwt);

                let header: Header<Empty> = token
                    .unverified_header()
                    .map_err(AuthError::JWTTokenError)?;

                let key_id = header.registered.key_id.ok_or(AuthError::JWKSError)?;

                debug!("Fetching signing key for '{:?}'", key_id);

                // Now that we have the key, construct our RSA public key secret
                let secret =
                    get_secret_from_key_set(jwks, &key_id).map_err(|_err| AuthError::JWKSError)?;

                // Now fully verify and extract the token
                let token = token
                    .into_decoded(&secret, SignatureAlgorithm::RS256)
                    .map_err(AuthError::JWTTokenError)?;

                let payload = token.payload().map_err(AuthError::JWTTokenError)?;

                debug!(
                    "Successfully verified token with subject: {:?}",
                    payload.registered.subject
                );

                Ok(payload.clone())
            }
            JWKSValidator::Unverified => {
                let token = JWT::<Empty, Empty>::new_encoded(jwt);

                let payload = &token
                    .unverified_payload()
                    .map_err(AuthError::JWTTokenError)?;

                Ok(payload.clone())
            }
        }
    }
}

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

/// Provide the Json Web Key Set
///
/// **Provides:** `Arc<jwks::JWKS>`
///
/// **Depends on:**
///   - `<C: Config>` - requires that `C` fulfills the `AuthConfig: FromRef<C>` constraint
#[derive(Default)]
pub struct ProvideJwks<C: Config> {
    config_tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<C>,
}

impl<C: Config> ProvideJwks<C> {
    /// Create a new instance of ProvideJwks
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
impl<C: Config> Provider<JWKSet<Empty>> for ProvideJwks<C>
where
    AuthConfig: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<JWKSet<Empty>>> {
        let config = if let Some(tag) = self.config_tag {
            i.get(tag).await?
        } else {
            i.get_type::<C>().await?
        };

        let auth = AuthConfig::from_ref(&*config);
        let key_set = init(auth).await;

        Ok(Arc::new(key_set))
    }
}
