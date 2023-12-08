use std::ops::Deref;

use axum::extract::FromRef;
use nakago::{hooks, Config};
use nakago_axum::{self, auth, AxumApplication};

use super::http::GraphQL;

/// Test utils, extended for application-specific helpers
pub struct Utils<C: Config> {
    utils: nakago_axum::test::utils::Utils<C>,

    /// GraphQL test utils
    pub graphql: GraphQL,
}

impl<C: Config> Deref for Utils<C> {
    type Target = nakago_axum::test::utils::Utils<C>;

    fn deref(&self) -> &Self::Target {
        &self.utils
    }
}

impl<C: Config> Utils<C> {
    /// Initialize the GraphQL test utils
    pub async fn init(
        app: AxumApplication<C>,
        config_path: &str,
        base_url: &str,
        graphql_url: &str,
    ) -> hooks::Result<Self>
    where
        C: Config,
        nakago_axum::Config: FromRef<C>,
        auth::Config: FromRef<C>,
    {
        let utils = nakago_axum::test::utils::Utils::init(app, config_path, base_url).await?;

        let graphql = GraphQL::new(utils.http.clone(), graphql_url.to_string());

        Ok(Self { utils, graphql })
    }
}
