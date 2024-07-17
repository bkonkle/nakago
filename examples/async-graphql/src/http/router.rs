use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::{init::trace_layer, State};
use nakago_ws::{connections, controller};

use crate::{domains::users::model::User, events::handler};

use super::{events, graphql, health};

/// Provide dependencies needed for the HTTP service
pub async fn load(i: &Inject) -> nakago::Result<()> {
    i.provide::<nakago_ws::Connections<User>>(connections::Provide::default())
        .await?;

    i.provide::<Box<dyn nakago_ws::Handler<User>>>(handler::Provide::default())
        .await?;

    i.provide::<nakago_ws::Controller<User>>(controller::Provide::default())
        .await?;

    Ok(())
}

/// Initialize the HTTP router
pub fn init(i: &Inject) -> Router {
    Router::new()
        .layer(trace_layer())
        .route("/health", get(health::health_check))
        .route("/graphql", get(graphql::graphiql).post(graphql::resolve))
        .route("/events", get(events::handle))
        .with_state(State::new(i.clone()))
}
