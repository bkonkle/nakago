use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;

use crate::domains::users;

use super::socket;

/// Tag(events::Controller)
pub const CONTROLLER: Tag<Controller> = Tag::new("events::Controller");

/// State for the WebSocket Events Handler
#[derive(Clone)]
pub struct Controller {
    users: Arc<Box<dyn users::Service>>,
    handler: Arc<socket::Handler>,
}

impl Controller {
    /// Create a new EventsState instance
    pub fn new(users: Arc<Box<dyn users::Service>>, handler: Arc<socket::Handler>) -> Self {
        Self { users, handler }
    }

    /// Handle WebSocket upgrade requests
    pub async fn upgrade(
        self,
        sub: Subject,
        ws: WebSocketUpgrade,
    ) -> axum::response::Result<impl IntoResponse> {
        // Retrieve the request User, if username is present
        let user = if let Subject(Some(ref username)) = sub {
            self.users
                .get_by_username(username, &true)
                .await
                .unwrap_or(None)
        } else {
            None
        };

        Ok(ws.on_upgrade(|socket| async move { self.handler.handle(socket, user).await }))
    }
}

#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Controller> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Controller>> {
        let users = i.get(&users::SERVICE).await?;
        let handler = i.get(&socket::HANDLER).await?;

        Ok(Arc::new(Controller::new(users, handler)))
    }
}
