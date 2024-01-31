use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use axum::{
    extract::{ws::Message, WebSocketUpgrade},
    response::IntoResponse,
};
use nakago::{provider, Inject, Provider, Tag};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;
use nakago_ws::{
    connections::Connections,
    socket::{self, Handler},
};

use crate::{
    domains::users::{self, model::User},
    messages::{IncomingMessage, OutgoingMessage},
};

/// Events Controller
pub const CONTROLLER: Tag<Controller> = Tag::new("events::Controller");

/// Connections
pub const CONNECTIONS: Tag<Connections<User>> = Tag::new("events::Connections");

/// Message Handler
pub const HANDLER: Tag<Handler<User>> = Tag::new("events::Handler");

/// Events Controller
#[derive(Clone)]
pub struct Controller {
    users: Arc<Box<dyn users::Service>>,
    handler: Arc<socket::Handler<User>>,
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

/// WebSocket Event Handler
pub struct Router {
    connections: Arc<Connections<User>>,
}

impl Router {
    async fn route(&self, conn_id: &str, msg: Message) -> anyhow::Result<()> {
        let message: IncomingMessage = msg.into();

        match message {
            IncomingMessage::Ping => self.handle_ping(conn_id).await,
            IncomingMessage::CannotDeserialize => Err(anyhow!("cannot deserialize message")),
        }
    }

    async fn handle_ping(&self, conn_id: &str) -> Result<()> {
        self.connections.send(conn_id, OutgoingMessage::Pong.into());

        Ok(())
    }
}

/// Events Provider
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Controller> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Controller>> {
        let users = i.get(&users::SERVICE).await?;
        let handler = i.get(&HANDLER).await?;

        Ok(Arc::new(Controller { users, handler }))
    }
}
