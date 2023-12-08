use async_trait::async_trait;
use hyper::Method;
use nakago::{hooks, Hook, Inject};
use nakago_axum::routes;

use super::{events, graphql, health};

/// Load dependencies for all handlers
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide(&graphql::CONTROLLER, graphql::Provide::default())
            .await?;

        i.provide(&events::CONTROLLER, events::Provide::default())
            .await?;

        Ok(())
    }
}

/// Init all handlers
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        let graphql_controller = i.get(&graphql::CONTROLLER).await?;
        let events_controller = i.get(&events::CONTROLLER).await?;

        i.handle(routes::Init::new(
            Method::GET,
            "/health",
            health::health_check,
        ))
        .await?;

        i.handle(routes::Init::new(
            Method::GET,
            "/graphql",
            graphql::Controller::graphiql,
        ))
        .await?;

        i.handle(routes::Init::new(
            Method::POST,
            "/graphql",
            move |sub, req| async move {
                graphql::Controller::resolve(graphql_controller, sub, req).await
            },
        ))
        .await?;

        i.handle(routes::Init::new(
            Method::GET,
            "/events",
            move |sub, ws| async move {
                events::Controller::upgrade(events_controller, sub, ws).await
            },
        ))
        .await?;

        Ok(())
    }
}
