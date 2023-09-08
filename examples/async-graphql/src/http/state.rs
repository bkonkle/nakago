use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_axum::{
    app::State,
    auth::{authenticate::AuthState, AUTH_STATE},
};
use nakago_derive::Provider;

use crate::{
    domains::users::service::USERS_SERVICE, events::SOCKET_HANDLER, graphql::GRAPHQL_SCHEMA,
};

use super::handlers::{EventsState, GraphQLState};

/// Tag(AppState)
pub const STATE: Tag<AppState> = Tag::new("AppState");

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct AppState {
    auth: AuthState,
    events: EventsState,
    graphql: GraphQLState,
}

impl State for AppState {}

/// Provide the AppState for Axum
///
/// **Provides:** `AppState`
///
/// **Depends on:**
///   - `Tag(AuthState)`
///   - `Tag(UsersService)`
///   - `Tag(SocketHandler)`
///   - `Tag(GraphQLSchema)`
#[derive(Default)]
pub struct ProvideAppState {}

#[Provider]
#[async_trait]
impl Provider<AppState> for ProvideAppState {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<AppState>> {
        let auth = i.get(&AUTH_STATE).await?;
        let users = i.get(&USERS_SERVICE).await?;
        let handler = i.get(&SOCKET_HANDLER).await?;
        let schema = i.get(&GRAPHQL_SCHEMA).await?;

        let events = EventsState::new(users.clone(), handler.clone());
        let graphql = GraphQLState::new(users, schema);

        Ok(Arc::new(AppState {
            auth: (*auth).clone(),
            events,
            graphql,
        }))
    }
}
