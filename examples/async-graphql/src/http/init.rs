use async_trait::async_trait;
use axum::{
    routing::{get, post},
    Router,
};
use nakago::{hooks, Hook, Inject};
use nakago_axum::routes;
use nakago_ws::{connections, controller};

use super::{events, graphql, health};

/// Load dependencies for all handlers
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide(&graphql::CONTROLLER, graphql::Provide::default())
            .await?;

        i.provide(&events::CONNECTIONS, connections::Provide::default())
            .await?;

        i.provide(&events::HANDLER, events::Provide::default())
            .await?;

        i.provide(
            &events::CONTROLLER,
            controller::Provide::new(Some(&events::CONNECTIONS), Some(&events::HANDLER)),
        )
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
            Router::new().route("/health", get(health::health_check)),
        ))
        .await?;

        i.handle(routes::Init::new(
            Router::new().route("/graphql", get(graphql::Controller::graphiql)),
        ))
        .await?;

        i.handle(routes::Init::new(Router::new().route(
            "/graphql",
            post(move |sub, req| async move { graphql_controller.resolve(sub, req).await }),
        )))
        .await?;

        i.handle(routes::Init::new(Router::new().route(
            "/events",
            get(move |sub, ws| async move { events_controller.upgrade(sub, ws).await }),
        )))
        .await?;

        Ok(())
    }
}
