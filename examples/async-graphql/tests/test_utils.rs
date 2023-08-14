#![allow(dead_code)] // Since each test is an independent module, this is needed

use std::{default::Default, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use axum::{extract::FromRef, http::HeaderValue};
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{RegisteredHeader, Secret},
    ClaimsSet, Empty, RegisteredClaims, SingleOrMultiple, JWT,
};
use fake::{Fake, Faker};
use futures_util::{stream::SplitStream, Future, SinkExt, StreamExt};
use hyper::{client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use nakago::{Dependency, EventType, Inject, InjectResult, Provider};
use nakago_axum::{auth::config::AuthConfig, AxumApplication};
use nakago_examples_async_graphql::{
    config::AppConfig,
    domains::{
        episodes::{model::Episode, mutations::CreateEpisodeInput, providers::EPISODES_SERVICE},
        profiles::{model::Profile, mutations::CreateProfileInput, providers::PROFILES_SERVICE},
        shows::{model::Show, mutations::CreateShowInput, providers::SHOWS_SERVICE},
        users::{model::User, providers::USERS_SERVICE},
    },
    providers::{InitApp, StartApp},
    routes::{init_events_route, init_graphql_route, init_health_route, AppState},
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::{
    net::TcpStream,
    time::{sleep, timeout},
};
use tokio_tungstenite::{
    connect_async, tungstenite,
    tungstenite::{client::IntoClientRequest, Message},
    MaybeTlsStream, WebSocketStream,
};

static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);

/// Creates an http/https client via Hyper
pub fn http_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder().build::<_, Body>(HttpsConnector::new())
}

/// Run the Application Server
pub async fn run_server() -> Result<(AxumApplication<AppConfig>, SocketAddr)> {
    let mut app = AxumApplication::<AppConfig>::default();
    app.on(&EventType::Init, InitApp::default());
    app.on(&EventType::Init, init_health_route());
    app.on(&EventType::Init, init_graphql_route());
    app.on(&EventType::Init, init_events_route());
    app.on(&EventType::Startup, StartApp::default());

    let server = app.run::<AppState>(None).await?;
    let addr = server.local_addr();

    // Spawn the server in the background
    tokio::spawn(server);

    // Wait for it to initialize
    sleep(Duration::from_millis(200)).await;

    // Return the bound address
    Ok((app, addr))
}

struct HttpClientProvider {}

#[async_trait]
impl Provider for HttpClientProvider {
    async fn provide(self: Arc<Self>, _i: Inject) -> InjectResult<Arc<Dependency>> {
        Ok(Arc::new(
            Client::builder().build::<_, Body>(HttpsConnector::new()),
        ))
    }
}

/// Common test utils
pub struct TestUtils {
    pub app: AxumApplication<AppConfig>,
    pub auth: AuthConfig,
    pub addr: SocketAddr,
    pub http_client: &'static Client<HttpsConnector<HttpConnector>>,
    pub graphql: GraphQL,
}

impl TestUtils {
    /// Initialize a new set of utils
    pub async fn init() -> Result<Self> {
        let (app, addr) = run_server().await?;

        let config = app.get_type::<AppConfig>().await?;

        let auth = AuthConfig::from_ref(&*config);

        let graphql = GraphQL::new(format!(
            "http://localhost:{port}/graphql",
            port = addr.port()
        ));

        let http_client = &HTTP_CLIENT;

        Ok(TestUtils {
            app,
            addr,
            auth,
            http_client,
            graphql,
        })
    }

    /// Create a test JWT token with a dummy secret
    pub fn create_jwt(&self, username: &str) -> String {
        let expected_claims = ClaimsSet::<Empty> {
            registered: RegisteredClaims {
                issuer: Some(self.auth.url.clone()),
                subject: Some(username.to_string()),
                audience: Some(SingleOrMultiple::Single(self.auth.audience.clone())),
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

        token.unwrap_encoded().to_string()
    }

    /// Create a User and Profile together
    #[allow(dead_code)] // Since each test is an independent module, this is necessary
    pub async fn create_user_and_profile(
        &self,
        username: &str,
        email: &str,
    ) -> Result<(User, Profile)> {
        let users = self.app.get(&USERS_SERVICE).await?;
        let user = users.create(username).await?;

        let mut profile_input: CreateProfileInput = Faker.fake();
        profile_input.user_id = user.id.clone();
        profile_input.email = email.to_string();

        let profiles = self.app.get(&PROFILES_SERVICE).await?;
        let profile = profiles.create(&profile_input, &false).await?;

        Ok((user, profile))
    }

    /// Create a Show and Episode together
    #[allow(dead_code)] // Since each test is an independent module, this is necessary
    pub async fn create_show_and_episode(
        &self,
        show_title: &str,
        episode_title: &str,
    ) -> Result<(Show, Episode)> {
        let show_input = CreateShowInput {
            title: show_title.to_string(),
            ..Default::default()
        };

        let shows = self.app.get(&SHOWS_SERVICE).await?;
        let show = shows.create(&show_input).await?;

        let episode_input = CreateEpisodeInput {
            title: episode_title.to_string(),
            show_id: show.id.clone(),
            ..Default::default()
        };

        let episodes = self.app.get(&EPISODES_SERVICE).await?;
        let episode = episodes.create(&episode_input, &false).await?;

        Ok((show, episode))
    }

    /// Send a message with the default timeout
    pub async fn send_message<F, T>(
        &self,
        message: Message,
        token: Option<&str>,
        to_future: F,
    ) -> Result<()>
    where
        F: Fn(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> T,
        T: Future,
    {
        self.send_to_websocket(message, token, to_future, None)
            .await
    }

    /// Send a message with a custom timeout
    pub async fn send_message_with_timeout<F, T>(
        &self,
        message: Message,
        token: Option<&str>,
        to_future: F,
        timer: u64,
    ) -> Result<()>
    where
        F: Fn(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> T,
        T: Future,
    {
        self.send_to_websocket(message, token, to_future, Some(timer))
            .await
    }

    async fn send_to_websocket<F, T>(
        &self,
        message: Message,
        token: Option<&str>,
        to_future: F,
        time: Option<u64>,
    ) -> Result<()>
    where
        F: Fn(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> T,
        T: Future,
    {
        let mut req = url::Url::parse(&format!(
            "ws://localhost:{port}/events",
            port = self.addr.port()
        ))?
        .into_client_request()?;

        if let Some(token) = token {
            let headers = req.headers_mut();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        }

        let result = connect_async(req).await;

        if let Err(err) = result {
            let error = if let tungstenite::Error::Http(response) = &err {
                #[derive(Deserialize, Debug)]
                struct JsonError {
                    code: Option<String>,
                    error: Option<String>,
                }

                let body: Option<JsonError> = response
                    .body()
                    .as_ref()
                    .and_then(|body| serde_json::from_slice(body).unwrap_or(None));

                if let Some(error) = body {
                    error.error.unwrap_or(format!("{:?}", err))
                } else {
                    err.to_string()
                }
            } else {
                err.to_string()
            };

            panic!("Failed to connect: {error}");
        }

        let (ws_stream, _) = result.unwrap();
        let (mut write, read) = ws_stream.split();

        write.send(message).await.unwrap();

        if timeout(Duration::from_millis(time.unwrap_or(1000)), to_future(read))
            .await
            .is_err()
        {
            panic!("Error: future timed out")
        }

        Ok(())
    }
}

/// Utilities for testing graphql endpoints
pub struct GraphQL {
    url: String,
}

impl GraphQL {
    /// Construct a new GraphQL helper with a path to the endpoint
    pub fn new(url: String) -> Self {
        GraphQL { url }
    }

    /// Create a GraphQL query request for Hyper with an optional auth token
    pub fn query(
        &self,
        query: &str,
        variables: Value,
        token: Option<&str>,
    ) -> Result<Request<Body>> {
        let mut req = Request::builder().method(Method::POST).uri(&self.url);

        if let Some(token) = token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let body = serde_json::to_string(&json!({ "query": query, "variables": variables }))?;

        req.body(Body::from(body)).map_err(|err| err.into())
    }
}
