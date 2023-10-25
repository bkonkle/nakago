use std::ops::Deref;

use axum::extract::FromRef;
use nakago::{Config, InjectResult};
use nakago_axum::{self, app::State, auth, AxumApplication};

use super::http::GraphQL;

/// Test utils, extended for application-specific helpers
pub struct Utils<C: Config, S: State> {
    utils: nakago_axum::test::utils::Utils<C, S>,

    /// GraphQL test utils
    pub graphql: GraphQL,
}

impl<C: Config, S: State> Deref for Utils<C, S> {
    type Target = nakago_axum::test::utils::Utils<C, S>;

    fn deref(&self) -> &Self::Target {
        &self.utils
    }
}

impl<C: Config, S: State> Utils<C, S> {
    /// Initialize the GraphQL test utils
    pub async fn init(
        app: AxumApplication<C, S>,
        config_path: &str,
        base_url: &str,
        graphql_url: &str,
    ) -> InjectResult<Self>
    where
        C: Config,
        S: State,
        nakago_axum::Config: FromRef<C>,
        auth::Config: FromRef<C>,
    {
        let utils =
            nakago_axum::test::utils::Utils::init(app, config_path, base_url.clone()).await?;

        let base_url = if graphql_url.starts_with('/') {
            base_url.strip_suffix('/').unwrap_or(base_url)
        } else {
            base_url
        };

        let graphql = GraphQL::new(format!(
            "http://localhost:{port}{base_url}{graphql_url}",
            port = utils.addr.port()
        ));

        Ok(Self { utils, graphql })
    }
}
