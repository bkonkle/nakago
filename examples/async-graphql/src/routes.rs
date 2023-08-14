use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::FromRef, routing::get, Router};
use nakago::{Dependency, Inject, InjectResult, Provider};
use nakago_axum::{
    app::State,
    auth::{authenticate::AuthState, AUTH_STATE},
    InitRoute, Route,
};

use crate::{
    domains::users::service::USERS_SERVICE, events::SOCKET_HANDLER, graphql::GRAPHQL_SCHEMA,
    handlers::GraphQLState,
};

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

/// Provide the AppState for Axum
///
/// **Provides:** `AppState`
///
/// **Depends on:**
///   - `Tag(AuthState)`
///   - `Tag(UsersService)`
///   - `Tag(SocketHandler)`
#[derive(Default)]
pub struct ProvideAppState {}

#[async_trait]
impl Provider for ProvideAppState {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let auth = i.get(&AUTH_STATE).await?;
        let users = i.get(&USERS_SERVICE).await?;
        let handler = i.get(&SOCKET_HANDLER).await?;
        let schema = i.get(&GRAPHQL_SCHEMA).await?;

        let events = EventsState::new(users.clone(), handler.clone());
        let graphql = GraphQLState::new(users, schema);

        Ok(Arc::new(AppState::new((*auth).clone(), events, graphql)))
    }
}
