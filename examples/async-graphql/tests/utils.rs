#![allow(dead_code)] // Since each test is an independent module, this is needed

use std::{default::Default, ops::Deref, time::Duration};

use anyhow::Result;
use axum::http::HeaderValue;
use fake::{Fake, Faker};
use futures_util::{stream::SplitStream, Future, SinkExt, StreamExt};
use nakago_axum::auth::{validator, Validator};
use serde::Deserialize;
use tokio::{net::TcpStream, time::timeout};
use tokio_tungstenite::{
    connect_async, tungstenite,
    tungstenite::{client::IntoClientRequest, Message},
    MaybeTlsStream, WebSocketStream,
};

use nakago_examples_async_graphql::{
    domains::{
        episodes::{self, model::Episode, mutation::CreateEpisodeInput},
        profiles::{self, model::Profile, mutation::CreateProfileInput},
        shows::{self, model::Show, mutation::CreateShowInput},
        users::{self, model::User},
    },
    init, Config,
};

/// Test utils, extended for application-specific helpers
pub struct Utils(nakago_async_graphql::test::Utils<Config>);

impl Deref for Utils {
    type Target = nakago_async_graphql::test::Utils<Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Utils {
    pub async fn init() -> Result<Self> {
        let app = init::app().await?;

        app.replace_with::<Validator>(validator::ProvideUnverified::default())
            .await?;

        let config_path = std::env::var("CONFIG_PATH_ASYNC_GRAPHQL")
            .unwrap_or_else(|_| "examples/async-graphql/config.test.toml".to_string());

        let utils =
            nakago_async_graphql::test::Utils::init(app, &config_path, "/", "/graphql").await?;

        Ok(Self(utils))
    }

    /// Create a User and Profile together
    #[allow(dead_code)] // Since each test is an independent module, this is necessary
    pub async fn create_user_and_profile(
        &self,
        username: &str,
        email: &str,
    ) -> Result<(User, Profile)> {
        let users = self.app.get::<Box<dyn users::Service>>().await?;
        let user = users.create(username).await?;

        let mut profile_input: CreateProfileInput = Faker.fake();
        profile_input.user_id.clone_from(&user.id);
        profile_input.email = email.to_string();

        let profiles = self.app.get::<Box<dyn profiles::Service>>().await?;
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

        let shows = self.app.get::<Box<dyn shows::Service>>().await?;
        let show = shows.create(&show_input).await?;

        let episode_input = CreateEpisodeInput {
            title: episode_title.to_string(),
            show_id: show.id.clone(),
            ..Default::default()
        };

        let episodes = self.app.get::<Box<dyn episodes::Service>>().await?;
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
            let value = HeaderValue::from_str(&format!("Bearer {}", token))?;

            let headers = req.headers_mut();
            headers.insert("Authorization", value);
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
