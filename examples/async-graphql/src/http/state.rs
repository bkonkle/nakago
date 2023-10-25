use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::{self, auth};
use nakago_derive::Provider;

use crate::{domains::users, events::SOCKET_HANDLER, graphql};

use super::handlers::{EventsState, GraphQLState};

/// Tag(app::State)
pub const STATE: Tag<State> = Tag::new("app::State");

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct State {
    auth: auth::State,
    events: EventsState,
    graphql: GraphQLState,
}

impl nakago_axum::State for State {}

/// Provide the State for Axum
///
/// **Provides:** `app::State`
///
/// **Depends on:**
///   - `Tag(auth::State)`
///   - `Tag(users::Service)`
///   - `Tag(events::SocketHandler)`
///   - `Tag(praphql::Schema)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<State> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<State>> {
        let auth = i.get(&auth::STATE).await?;
        let users = i.get(&users::SERVICE).await?;
        let handler = i.get(&SOCKET_HANDLER).await?;
        let schema = i.get(&graphql::SCHEMA).await?;

        let events = EventsState::new(users.clone(), handler.clone());
        let graphql = GraphQLState::new(users, schema);

        Ok(Arc::new(State {
            auth: (*auth).clone(),
            events,
            graphql,
        }))
    }
}
