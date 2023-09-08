use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::{State, WebSocketUpgrade},
    response::{Html, IntoResponse},
    Json,
};
use nakago_axum::auth::Subject;
use serde::{Deserialize, Serialize};

use crate::{domains::users::service::UsersService, events::SocketHandler, graphql::GraphQLSchema};

// Health
// ------

/// A Health Check Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// The Status code
    code: usize,

    /// Whether the check was successful or not
    success: bool,
}

/// Handle health check requests
pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        code: 200,
        success: true,
    })
}

// GraphQL
// -------

/// State for the GraphQL Handler
#[derive(Clone)]
pub struct GraphQLState {
    users: Arc<Box<dyn UsersService>>,
    schema: Arc<GraphQLSchema>,
}

impl GraphQLState {
    /// Create a new GraphQLState instance
    pub fn new(users: Arc<Box<dyn UsersService>>, schema: Arc<GraphQLSchema>) -> Self {
        Self { users, schema }
    }
}

/// Handle GraphiQL Requests
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

/// Handle GraphQL Requests
pub async fn graphql_handler(
    State(state): State<GraphQLState>,
    sub: Subject,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        state
            .users
            .get_by_username(username, &true)
            .await
            .unwrap_or(None)
    } else {
        None
    };

    // Add the Subject and optional User to the context
    let request = req.into_inner().data(sub).data(user);

    state.schema.execute(request).await.into()
}

// Events
// ------

/// State for the WebSocket Events Handler
#[derive(Clone)]
pub struct EventsState {
    users: Arc<Box<dyn UsersService>>,
    handler: Arc<SocketHandler>,
}

impl EventsState {
    /// Create a new EventsState instance
    pub fn new(users: Arc<Box<dyn UsersService>>, handler: Arc<SocketHandler>) -> Self {
        Self { users, handler }
    }
}

/// Handle WebSocket upgrade requests
pub async fn events_handler(
    State(state): State<EventsState>,
    sub: Subject,
    ws: WebSocketUpgrade,
) -> axum::response::Result<impl IntoResponse> {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        state
            .users
            .get_by_username(username, &true)
            .await
            .unwrap_or(None)
    } else {
        None
    };

    Ok(ws.on_upgrade(|socket| async move { state.handler.handle(socket, user).await }))
}
