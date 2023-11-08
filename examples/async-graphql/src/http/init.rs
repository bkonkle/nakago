use async_trait::async_trait;
use hyper::Method;
use nakago::{inject, Hook, Inject};
use nakago_axum::routes;

use super::{events, graphql, health};

/// Init all handlers
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        i.handle(routes::Init::new(
            Method::GET,
            "/health",
            health::health_check,
        ))
        .await?;

        i.handle(routes::Init::new(
            Method::GET,
            "/graphql",
            graphql::graphiql,
        ))
        .await?;

        i.handle(routes::Init::new(
            Method::POST,
            "/graphql",
            graphql::resolve,
        ))
        .await?;

        i.handle(routes::Init::new(Method::GET, "/events", events::upgrade))
            .await?;

        Ok(())
    }
}
