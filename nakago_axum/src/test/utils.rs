use std::{default::Default, net::SocketAddr, sync::Arc, time::Duration};

use axum::extract::FromRef;
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{RegisteredHeader, Secret},
    ClaimsSet, Empty, RegisteredClaims, SingleOrMultiple, JWT,
};
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use nakago::{Config, InjectResult};
use tokio::time::sleep;

use crate::{app::State, auth::config::AuthConfig, config::HttpConfig, AxumApplication};

use super::{http::HTTP_CLIENT, HttpClientProvider};

/// Common test utils
pub struct TestUtils<C, S>
where
    C: Config,
    S: State,
{
    /// The Application instance
    pub app: AxumApplication<C, S>,

    /// The Address the server is listening on
    pub addr: SocketAddr,

    /// The test HTTP client
    pub http_client: Arc<Client<HttpsConnector<HttpConnector>>>,
}

impl<C, S> TestUtils<C, S>
where
    C: Config,
    S: State,
    HttpConfig: FromRef<C>,
    AuthConfig: FromRef<C>,
{
    /// Initialize a new set of utils
    pub async fn init<F>(init_app: F) -> InjectResult<Self>
    where
        F: Fn() -> AxumApplication<C, S>,
    {
        let app = init_app();

        app.provide(&HTTP_CLIENT, HttpClientProvider::default())
            .await?;

        let server = app.run(None).await?;
        let addr = server.local_addr();

        // Spawn the server in the background
        tokio::spawn(server);

        // Wait for it to initialize
        sleep(Duration::from_millis(200)).await;

        let http_client = app.get(&HTTP_CLIENT).await?;

        Ok(TestUtils {
            app,
            addr,
            http_client,
        })
    }

    /// Create a test JWT token with a dummy secret
    pub async fn create_jwt(&self, username: &str) -> InjectResult<String> {
        let config = self.app.get_config().await?;
        let auth = AuthConfig::from_ref(&*config);

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
