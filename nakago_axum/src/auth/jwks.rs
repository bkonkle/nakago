use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use axum::extract::FromRef;
use biscuit::{
    jwk::{AlgorithmParameters, JWKSet, JWK},
    jws::Secret,
};
use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::{body::Buf, Request};
use hyper_util::rt::TokioIo;
use nakago::{self, inject, Inject, Provider, Tag};
use nakago_derive::Provider;
use thiserror::Error;
use tokio::net::TcpStream;

use super::Config;

/// The JWKS Tag
pub const JWKS: Tag<JWKSet<biscuit::Empty>> = Tag::new("auth::JWKS");

/// Get the default set of JWKS keys
pub async fn init(config: Config) -> JWKSet<biscuit::Empty> {
    let jwks_client = Client::new(config);

    jwks_client
        .get_key_set()
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
    pub async fn get_key_set(&self) -> anyhow::Result<JWKSet<biscuit::Empty>> {
        let url: hyper::Uri = format!("{}/.well-known/jwks.json", &self.config.url).parse()?;

        debug!("Fetching keys from '{}'", url);

        let host = url.host().expect("uri has no host");
        let port = url.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("JWKS connection failed: {:?}", err);
            }
        });

        let authority = url.authority().unwrap().clone();

        let req = Request::builder()
            .uri(url)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;

        let res = sender.send_request(req).await?;

        let body = res.collect().await?.aggregate();

        let jwks = serde_json::from_reader(body.reader())?;

        Ok(jwks)
    }
}

/// A convenience function to get a particular key from a key set, and convert it into a secret
pub fn get_secret_from_key_set(
    jwks: &JWKSet<biscuit::Empty>,
    key_id: &str,
) -> Result<Secret, ClientError> {
    let jwk = get_key(jwks, key_id)?;
    let secret = get_secret(jwk)?;

    Ok(secret)
}

/// Get a particular key from a key set by id
pub fn get_key(
    jwks: &JWKSet<biscuit::Empty>,
    key_id: &str,
) -> Result<JWK<biscuit::Empty>, ClientError> {
    let key = jwks.find(key_id).ok_or(ClientError::MissingKeyId)?.clone();

    Ok(key)
}

/// Convert a JWK into a Secret
pub fn get_secret(jwk: JWK<biscuit::Empty>) -> Result<Secret, ClientError> {
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
/// **Provides:** `Arc<JWKSet<biscuit::Empty>>`
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
impl<C: nakago::Config> Provider<JWKSet<biscuit::Empty>> for Provide<C>
where
    Config: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<JWKSet<biscuit::Empty>>> {
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
