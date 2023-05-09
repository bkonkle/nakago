use axum::{extract::FromRef, routing::get, Router};
use nakago_axum::{app::State, auth::authenticate::AuthState, InitRoute, Route};

use crate::handlers::GraphQLState;

use super::handlers::{events_handler, graphiql, graphql_handler, health_handler, EventsState};

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct AppState {
    auth: AuthState,
    events: EventsState,
    graphql: GraphQLState,
}

impl AppState {
    /// Create a new AppState instance
    pub fn new(auth: AuthState, events: EventsState, graphql: GraphQLState) -> Self {
        Self {
            auth,
            events,
            graphql,
        }
    }
}

impl State for AppState {}

/// Initialize the Health Route
pub fn init_health_route() -> InitRoute<AppState> {
    InitRoute::new(|_| Route::new("/", Router::new().route("/health", get(health_handler))))
}

/// Initialize the GraphQL Route
pub fn init_graphql_route() -> InitRoute<AppState> {
    InitRoute::new(|_| {
        Route::new(
            "/",
            Router::new().route("/graphql", get(graphiql).post(graphql_handler)),
        )
    })
}

/// Initialize the Events Route
pub fn init_events_route() -> InitRoute<AppState> {
    InitRoute::new(|_| Route::new("/", Router::new().route("/events", get(events_handler))))
}
