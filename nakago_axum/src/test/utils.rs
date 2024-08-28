use std::{
    default::Default, future::IntoFuture, net::SocketAddr, panic, sync::Arc, time::Duration,
};

use axum::Router;
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{RegisteredHeader, Secret},
    ClaimsSet, Empty, RegisteredClaims, SingleOrMultiple, JWT,
};
use nakago::{self, Tag};
use nakago_figment::FromRef;
use tokio::time::sleep;

use crate::{
    auth,
    init::{handle_panic, rust_log_subscriber, Listener},
    Config,
};

use super::http::Http;

/// Common test utils
pub struct Utils<C: nakago_figment::Config> {
    /// The Nakago Inject container
    pub i: nakago::Inject,

    /// The Address the server is listening on
    pub addr: SocketAddr,

    /// The test HTTP Request helper
    pub http: Arc<Http>,

    /// The config tag to use
    pub config_tag: Option<&'static Tag<C>>,
}

impl<C: nakago_figment::Config> Utils<C> {
    /// Initialize a new set of utils with no tags
    pub async fn init(i: nakago::Inject, base_url: &str, router: Router) -> nakago::Result<Self>
    where
        Config: FromRef<C>,
    {
        Utils::<C>::new(i, base_url, router, None).await
    }

    /// Initialize a new set of utils
    pub async fn new(
        i: nakago::Inject,
        base_url: &str,
        router: Router,
        config_tag: Option<&'static Tag<C>>,
    ) -> nakago::Result<Self>
    where
        Config: FromRef<C>,
    {
        panic::set_hook(Box::new(handle_panic));
        rust_log_subscriber();

        let (server, addr) = Listener::default().init(&i, router).await?;

        // Spawn the server in the background
        tokio::spawn(server.into_future());

        // Wait for it to initialize
        sleep(Duration::from_millis(200)).await;

        let http = Arc::new(Http::new(format!(
            "http://localhost:{port}{base_url}",
            port = addr.port(),
            base_url = base_url,
        )));

        Ok(Utils {
            i,
            addr,
            http,
            config_tag,
        })
    }

    /// Create a test JWT token with a dummy secret
    pub async fn create_jwt(&self, username: &str) -> nakago::Result<String>
    where
        auth::Config: FromRef<C>,
    {
        let config = if let Some(tag) = self.config_tag {
            self.i.get_tag(tag).await?
        } else {
            self.i.get::<C>().await?
        };

        let auth = auth::Config::from_ref(&*config);

        let expected_claims = ClaimsSet::<Empty> {
            registered: RegisteredClaims {
                issuer: Some(auth.url.clone()),
                subject: Some(username.to_string()),
                audience: Some(SingleOrMultiple::Single(auth.audience.clone())),
                ..Default::default()
            },
            private: Default::default(),
        };

        let jwt = JWT::new_decoded(
            From::from(RegisteredHeader {
                algorithm: SignatureAlgorithm::HS256,
                ..Default::default()
            }),
            expected_claims,
        );

        let token = jwt
            .into_encoded(&Secret::Bytes("test-jwt-secret".into()))
            .unwrap();

        Ok(token.unwrap_encoded().to_string())
    }
}
