use axum::{extract::FromRef, routing::get, Router};
use nakago_axum::{app::State, auth::authenticate::AuthState};

use super::handlers::{events_handler, health_handler, EventsState};

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct AppState {
    auth: AuthState,
    events: EventsState,
}

impl AppState {
    /// Create a new AppState instance
    pub fn new(auth: AuthState, events: EventsState) -> Self {
        Self { auth, events }
    }
}

impl State for AppState {}

/// Initialize the top-level app Router
pub fn init() -> Router<AppState> {
    Router::new()
        .nest("/health", Router::new().route("/", get(health_handler)))
        .nest("/events", Router::new().route("/", get(events_handler)))
}
