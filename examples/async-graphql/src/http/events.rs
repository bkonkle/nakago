use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use axum::extract::ws::Message;
use nakago::{provider, Inject, Provider, Tag};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;
use nakago_ws::{connections::Connections, handler::Handler};

use crate::{
    domains::users::{self, model::User},
    messages::{IncomingMessage, OutgoingMessage},
};

/// Connections
pub const CONNECTIONS: Tag<Connections<User>> = Tag::new("events::Connections");

/// Nakago WS Controller
pub const CONTROLLER: Tag<Box<dyn nakago_ws::Controller<User>>> = Tag::new("nakago_ws::Controller");

/// Message Handler
pub const HANDLER: Tag<Handler<User>> = Tag::new("events::Handler");

/// Events Controller
#[derive(Clone)]
pub struct Controller {
    connections: Arc<Connections<User>>,
    users: Arc<Box<dyn users::Service>>,
}

#[async_trait]
impl nakago_ws::Controller<User> for Controller {
    async fn get_user(&self, sub: Subject) -> Option<User> {
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

impl Controller {
    /// Handle a Ping message
    async fn handle_ping(&self, conn_id: &str) -> Result<()> {
        self.connections
            .send(conn_id, OutgoingMessage::Pong.into())
            .await?;

        Ok(())
    }
}

/// Events Provider
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Box<dyn nakago_ws::Controller<User>>> for Provide {
    async fn provide(
        self: Arc<Self>,
        i: Inject,
    ) -> provider::Result<Arc<Box<dyn nakago_ws::Controller<User>>>> {
        let connections = i.get(&CONNECTIONS).await?;
        let users = i.get(&users::SERVICE).await?;

        Ok(Arc::new(Box::new(Controller { connections, users })))
    }
}
