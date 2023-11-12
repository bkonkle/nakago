use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;

use crate::{domains::users, events::socket};

/// Events Controller
pub const CONTROLLER: Tag<Controller> = Tag::new("events::Controller");

/// Events Controller
#[derive(Clone)]
pub struct Controller {
    users: Arc<Box<dyn users::Service>>,
    handler: Arc<socket::Handler>,
}

impl Controller {
    /// Create a new Events handler
    pub async fn upgrade(
        self: Arc<Self>,
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

/// Events Provider
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Controller> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Controller>> {
        let users = i.get(&users::SERVICE).await?;
        let handler = i.get(&socket::HANDLER).await?;

        Ok(Arc::new(Controller { users, handler }))
    }
}
