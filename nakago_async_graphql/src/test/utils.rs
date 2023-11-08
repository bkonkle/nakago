use std::ops::Deref;

use axum::extract::FromRef;
use nakago::{inject, Config};
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
    ) -> inject::Result<Self>
    where
        C: Config,
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
