use std::{default::Default, net::SocketAddr, sync::Arc, time::Duration};

use axum::extract::FromRef;
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{RegisteredHeader, Secret},
    ClaimsSet, Empty, RegisteredClaims, SingleOrMultiple, JWT,
};
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use nakago::{self, inject};
use tokio::time::sleep;

use crate::{app::State, auth, AxumApplication, Config};

use super::http::{Http, ProvideClient, CLIENT};

/// Common test utils
pub struct Utils<C, S>
where
    C: nakago::Config,
    S: State,
{
    /// The Application instance
    pub app: AxumApplication<C, S>,

    /// The Address the server is listening on
    pub addr: SocketAddr,

    /// The test HTTP Request helper
    pub http: Http,

    /// The test HTTP client
    pub http_client: Arc<Client<HttpsConnector<HttpConnector>>>,
}

impl<C, S> Utils<C, S>
where
    C: nakago::Config,
    S: State,
    Config: FromRef<C>,
    auth::Config: FromRef<C>,
{
    /// Initialize a new set of utils
    pub async fn init(
        app: AxumApplication<C, S>,
        config_path: &str,
        base_url: &str,
    ) -> inject::Result<Self> {
        app.provide(&CLIENT, ProvideClient::default()).await?;

        let server = app.run(Some(config_path.into())).await?;
        let addr = server.local_addr();

        // Spawn the server in the background
        tokio::spawn(server);

        // Wait for it to initialize
        sleep(Duration::from_millis(200)).await;

        let http = Http::new(format!(
            "http://localhost:{port}{base_url}",
            port = addr.port(),
            base_url = base_url,
        ));

        let http_client = app.get(&CLIENT).await?;

        Ok(Utils {
            app,
            addr,
            http,
            http_client,
        })
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
