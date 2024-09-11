use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::ws::Message;
use nakago::{provider, Provider};
use nakago_derive::Provider;
use nakago_ws::{auth::Token, connections::Connections};

use crate::domains::users;

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
impl nakago_ws::Handler<Session> for Handler {
    async fn get_session(&self, token: Token) -> Option<Session> {
        let sub = token.claims?.registered.subject;

        if let Some(ref username) = sub {
            self.users
                .get_by_username(username, &true)
                .await
                .map(Session::new)
                .ok()
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
impl Provider<Box<dyn nakago_ws::Handler<Session>>> for Provide {
    async fn provide(
        self: Arc<Self>,
        i: nakago::Inject,
    ) -> provider::Result<Arc<Box<dyn nakago_ws::Handler<Session>>>> {
        let connections = i.get::<Connections<Session>>().await?;
        let users = i.get::<Box<dyn users::Service>>().await?;

        let temp = Box::new(Handler { connections, users });

        Ok(Arc::new(temp))
    }
}
