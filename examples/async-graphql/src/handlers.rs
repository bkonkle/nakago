use std::sync::Arc;

use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
    Json,
};
use nakago_axum::auth::Subject;
use serde::{Deserialize, Serialize};

use crate::domains::users::service::{UsersService, UsersServiceTrait};

use super::events::SocketHandler;

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

// Events
// ------

/// State for the WebSocket Events Handler
#[derive(Clone)]
pub struct EventsState {
    users: Arc<UsersService>,
    handler: SocketHandler,
}

impl EventsState {
    /// Create a new EventsState instance
    pub fn new(users: &Arc<UsersService>, handler: SocketHandler) -> Self {
        Self {
            users: users.clone(),
            handler,
        }
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
