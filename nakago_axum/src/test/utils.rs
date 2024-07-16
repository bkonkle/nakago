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
use tokio::{net::TcpListener, time::sleep};
use tracing_subscriber::prelude::*;

use crate::{auth, utils::handle_panic, Config};

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
    /// Initialize a new set of utils
    pub async fn init(
        i: nakago::Inject,
        config_path: &str,
        config_tag: Option<&'static Tag<C>>,
        base_url: &str,
        router: Router,
    ) -> nakago::Result<Self>
    where
        Config: FromRef<C>,
    {
        nakago_figment::loaders::Init::<C>::default()
            .with_path(config_path.into())
            .init(&i)
            .await?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        // Process setup
        panic::set_hook(Box::new(handle_panic));

        let config = if let Some(tag) = config_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<C>().await?
        };

        let http = Config::from_ref(&*config);

        let addr: SocketAddr = format!("0.0.0.0:{}", http.port)
            .parse()
            .expect("Unable to parse bind address");

        let listener = TcpListener::bind(&addr)
            .await
            .unwrap_or_else(|_| panic!("Unable to bind to address: {}", addr));

        let actual_addr = listener
            .local_addr()
            .map_err(|e| nakago::Error::Any(Arc::new(e.into())))?;

        let server = axum::serve(listener, router);

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
            addr: actual_addr,
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
