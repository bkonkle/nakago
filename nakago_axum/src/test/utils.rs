use std::{default::Default, future::IntoFuture, net::SocketAddr, sync::Arc, time::Duration};

use axum::extract::FromRef;
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{RegisteredHeader, Secret},
    ClaimsSet, Empty, RegisteredClaims, SingleOrMultiple, JWT,
};
use nakago::{self, hooks, inject};
use tokio::time::sleep;

use crate::{auth, AxumApplication, Config};

use super::http::Http;

/// Common test utils
pub struct Utils<C>
where
    C: nakago::Config,
{
    /// The Application instance
    pub app: AxumApplication<C>,

    /// The Address the server is listening on
    pub addr: SocketAddr,

    /// The test HTTP Request helper
    pub http: Arc<Http>,
}

impl<C> Utils<C>
where
    C: nakago::Config,
    Config: FromRef<C>,
    auth::Config: FromRef<C>,
{
    /// Initialize a new set of utils
    pub async fn init(
        app: AxumApplication<C>,
        config_path: &str,
        base_url: &str,
    ) -> hooks::Result<Self> {
        let (server, addr) = app.run(Some(config_path.into())).await?;

        // Spawn the server in the background
        tokio::spawn(server.into_future());

        // Wait for it to initialize
        sleep(Duration::from_millis(200)).await;

        let http = Arc::new(Http::new(format!(
            "http://localhost:{port}{base_url}",
            port = addr.port(),
            base_url = base_url,
        )));

        Ok(Utils { app, addr, http })
    }

    /// Create a test JWT token with a dummy secret
    pub async fn create_jwt(&self, username: &str) -> inject::Result<String> {
        let config = self.app.get_config().await?;
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
