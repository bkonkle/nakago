use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::ws::Message;
use nakago::{provider, Provider};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;
use nakago_ws::connections::Connections;

use crate::domains::users::{self, model::User};

use super::{
    messages::{IncomingMessage, OutgoingMessage},
    session::Session,
};

/// Message Handler
#[derive(Clone)]
pub struct Handler {
    connections: Arc<Connections<Session>>,
    users: Arc<Box<dyn users::Service>>,
}

#[async_trait]
impl nakago_ws::Handler<User> for Handler {
    async fn get_session(&self, sub: Subject) -> Option<User> {
        if let Subject(Some(ref username)) = sub {
            self.users
                .get_by_username(username, &true)
                .await
                .unwrap_or(None)
        } else {
            None
        }
    }

    async fn route(&self, conn_id: &str, msg: Message) -> anyhow::Result<()> {
        let message: IncomingMessage = msg.into();

        match message {
            IncomingMessage::Ping => self.handle_ping(conn_id).await,
            IncomingMessage::CannotDeserialize => Err(anyhow!("cannot deserialize message")),
        }
    }
}

impl Handler {
    /// Handle a Ping message
    async fn handle_ping(&self, conn_id: &str) -> anyhow::Result<()> {
        self.connections
            .send(conn_id, OutgoingMessage::Pong.into())
            .await?;

        Ok(())
    }
}

/// Message Handler Provider
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Box<dyn nakago_ws::Handler<User>>> for Provide {
    async fn provide(
        self: Arc<Self>,
        i: nakago::Inject,
    ) -> provider::Result<Arc<Box<dyn nakago_ws::Handler<User>>>> {
        let connections = i.get::<Connections<Session>>().await?;
        let users = i.get::<Box<dyn users::Service>>().await?;

        Ok(Arc::new(Box::new(Handler { connections, users })))
    }
}
