use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use nakago_axum::{auth::Subject, state, Error};

use crate::{domains::users, events::socket};

/// Handle WebSocket upgrade requests
pub async fn upgrade(
    state::Inject(i): state::Inject,
    sub: Subject,
    ws: WebSocketUpgrade,
) -> axum::response::Result<impl IntoResponse> {
    let users = i.get(&users::SERVICE).await.map_err(Error)?;
    let handler = i.get(&socket::HANDLER).await.map_err(Error)?;

    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        users.get_by_username(username, &true).await.unwrap_or(None)
    } else {
        None
    };

    Ok(ws.on_upgrade(|socket| async move { handler.handle(socket, user).await }))
}
