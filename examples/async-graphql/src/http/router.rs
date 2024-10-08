use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::{init::trace_layer, State};
use nakago_ws::{connections, controller};

use crate::events::{handler, session::Session};

use super::{events, graphql, health};

/// Provide dependencies needed for the HTTP service
pub async fn load(i: &Inject) -> nakago::Result<()> {
    i.provide::<nakago_ws::Connections<Session>>(connections::Provide::default())
        .await?;

    i.provide::<Box<dyn nakago_ws::Handler<Session>>>(handler::Provide::default())
        .await?;

    i.provide::<nakago_ws::Controller<Session>>(controller::Provide::default())
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
