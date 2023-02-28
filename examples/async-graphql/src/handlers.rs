use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
    Json,
};
use nakago_axum::auth::Subject;
use serde::{Deserialize, Serialize};

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
    handler: SocketHandler,
}

impl EventsState {
    /// Create a new EventsState instance
    pub fn new(handler: SocketHandler) -> Self {
        Self { handler }
    }
}

/// Handle WebSocket upgrade requests
pub async fn events_handler(
    State(state): State<EventsState>,
    _sub: Subject,
    ws: WebSocketUpgrade,
) -> axum::response::Result<impl IntoResponse> {
    Ok(ws.on_upgrade(|socket| async move { state.handler.handle(socket).await }))
}
