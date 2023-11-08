use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::Route;

use crate::events;

use super::{
    health::{graphiql, graphql_handler, health_handler},
    State,
};

/// Initialize the Health Route
pub fn new_health_route(_: Inject) -> Route<State> {
    Route::new("/", Router::new().route("/health", get(health_handler)))
}

/// Initialize the GraphQL Route
pub fn new_graphql_route(_: Inject) -> Route<State> {
    Route::new(
        "/",
        Router::new().route("/graphql", get(graphiql).post(graphql_handler)),
    )
}

/// Initialize the Events Route
pub async fn new_events_route(i: Inject) -> Route<State> {
    let controller = i.get(&events::CONTROLLER).await;

    Route::new("/", Router::new().route("/events", get(events_handler)))
}
