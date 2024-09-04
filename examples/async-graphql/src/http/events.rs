use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use nakago_axum::Inject;
use nakago_ws::auth::Token;

use crate::events::session::Session;

/// Handle WebSocket Events
pub async fn handle(
    Inject(events_controller): Inject<nakago_ws::Controller<Session>>,
    token: Token,
    ws: WebSocketUpgrade,
) -> axum::response::Result<impl IntoResponse> {
    events_controller.upgrade(token, ws).await
}
