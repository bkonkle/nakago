use std::ops::Deref;

use axum::extract::FromRef;
use nakago::{Config, InjectResult};
use nakago_axum::{app::State, auth::config::AuthConfig, config::HttpConfig, AxumApplication};

use super::http::GraphQL;

/// Test utils, extended for application-specific helpers
pub struct TestUtils<C: Config, S: State> {
    utils: nakago_axum::test::utils::TestUtils<C, S>,

    /// GraphQL test utils
    pub graphql: GraphQL,
}

impl<C: Config, S: State> Deref for TestUtils<C, S> {
    type Target = nakago_axum::test::utils::TestUtils<C, S>;

    fn deref(&self) -> &Self::Target {
        &self.utils
    }
}

impl<C: Config, S: State> TestUtils<C, S> {
    /// Initialize the GraphQL test utils
    pub async fn init<F>(init_app: F) -> InjectResult<Self>
    where
        F: Fn() -> AxumApplication<C, S>,
        C: Config,
        S: State,
        HttpConfig: FromRef<C>,
        AuthConfig: FromRef<C>,
    {
        let utils = nakago_axum::test::utils::TestUtils::init(init_app).await?;

        let graphql = GraphQL::new(format!(
            "http://localhost:{port}/graphql",
            port = utils.addr.port()
        ));

        Ok(Self { utils, graphql })
    }
}
