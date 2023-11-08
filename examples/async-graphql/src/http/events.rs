use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::WebSocketUpgrade, response::IntoResponse, routing::get, Router};
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::{auth::Subject, state, Error, Route};
use nakago_derive::Provider;
use tokio::sync::Mutex;

use crate::{domains::users, events::socket};

/// Tag(events::upgrade::Route)
pub const UPGRADE_ROUTE: Tag<Route> = Tag::new("events::upgrade::Route");

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

/// A Provider for the WebSocket upgrade route
#[derive(Default)]
pub struct ProvideUpgrade {}

#[Provider]
#[async_trait]
impl Provider<Route> for ProvideUpgrade {
    async fn provide(self: Arc<Self>, _: Inject) -> inject::Result<Arc<Route>> {
        let route = Router::new().route("/events", get(upgrade));

        Ok(Arc::new(Mutex::new(route)))
    }
}
