use async_trait::async_trait;
use axum::{routing::get, Router};
use nakago::{hooks, Hook, Inject};
use nakago_axum::routes;
use nakago_ws::{connections, controller};

use crate::domains::users::model::User;

use super::{events, graphql, health};

/// Load dependencies for all handlers
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.provide::<nakago_ws::Connections<User>>(connections::Provide::default())
            .await?;

        i.provide::<Box<dyn nakago_ws::Handler<User>>>(events::Provide::default())
            .await?;

        i.provide::<nakago_ws::Controller<User>>(controller::Provide::default())
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
        let events_controller = i.get::<nakago_ws::Controller<User>>().await?;

        i.handle(routes::Init::new(
            Router::new()
                .route("/health", get(health::health_check))
                .route("/graphql", get(graphql::graphiql).post(graphql::resolve)),
        ))
        .await?;

        i.handle(routes::Init::new(Router::new().route(
            "/events",
            get(move |sub, ws| async move { events_controller.upgrade(sub, ws).await }),
        )))
        .await?;

        Ok(())
    }
}
