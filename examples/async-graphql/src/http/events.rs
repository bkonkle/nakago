use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use nakago_axum::{auth::Subject, Inject};

use crate::events::session::Session;

/// Handle WebSocket Events
pub async fn handle(
    Inject(events_controller): Inject<nakago_ws::Controller<Session>>,
    sub: Subject,
    ws: WebSocketUpgrade,
) -> axum::response::Result<impl IntoResponse> {
    events_controller.upgrade(sub, ws).await
}
