use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::Route;

use super::{
    handlers::{events_handler, graphiql, graphql_handler, health_handler},
    state::AppState,
};

/// Initialize the Health Route
pub fn new_health_route(_: Inject) -> Route<AppState> {
    Route::new("/", Router::new().route("/health", get(health_handler)))
}

/// Initialize the GraphQL Route
pub fn new_graphql_route(_: Inject) -> Route<AppState> {
    Route::new(
        "/",
        Router::new().route("/graphql", get(graphiql).post(graphql_handler)),
    )
}

/// Initialize the Events Route
pub fn new_events_route(_: Inject) -> Route<AppState> {
    Route::new("/", Router::new().route("/events", get(events_handler)))
}
