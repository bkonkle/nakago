use std::ops::Deref;

use axum::Router;
use nakago::Tag;
use nakago_axum;
use nakago_figment::{Config, FromRef};

use super::http::GraphQL;

/// Test utils, extended for application-specific helpers
pub struct Utils<C: Config> {
    utils: nakago_axum::test::Utils<C>,

    /// GraphQL test utils
    pub graphql: GraphQL,
}

impl<C: Config> Deref for Utils<C> {
    type Target = nakago_axum::test::Utils<C>;

    fn deref(&self) -> &Self::Target {
        &self.utils
    }
}

impl<C: Config> Utils<C> {
    /// Initialize the GraphQL test utils
    pub async fn init(
        i: nakago::Inject,
        base_url: &str,
        graphql_url: &str,
        router: Router,
    ) -> nakago::Result<Self>
    where
        C: Config,
        nakago_axum::Config: FromRef<C>,
    {
        let utils = nakago_axum::test::Utils::init(i.clone(), base_url, router).await?;

        let graphql = GraphQL::new(utils.http.clone(), graphql_url.to_string());

        Ok(Self { utils, graphql })
    }

    /// Initialize a new set of utils
    pub async fn new(
        i: nakago::Inject,
        base_url: &str,
        graphql_url: &str,
        router: Router,
        config_tag: Option<&'static Tag<C>>,
    ) -> nakago::Result<Self>
    where
        C: Config,
        nakago_axum::Config: FromRef<C>,
    {
        let utils = nakago_axum::test::Utils::new(i.clone(), base_url, router, config_tag).await?;

        let graphql = GraphQL::new(utils.http.clone(), graphql_url.to_string());

        Ok(Self { utils, graphql })
    }
}
