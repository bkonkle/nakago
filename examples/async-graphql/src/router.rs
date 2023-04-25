use axum::{extract::FromRef, routing::get, Router};
use nakago_axum::{app::State, auth::authenticate::AuthState};

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

/// Initialize the top-level app Router
pub fn init() -> Router<AppState> {
    Router::new()
        .nest("/health", Router::new().route("/", get(health_handler)))
        .route("/graphql", get(graphiql).post(graphql_handler))
        .nest("/events", Router::new().route("/", get(events_handler)))
}
