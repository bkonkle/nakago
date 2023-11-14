use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use axum::extract::ws::WebSocket;
use derive_new::new;
use futures::{SinkExt, StreamExt, TryFutureExt};
use log::error;
use nakago::{inject, Inject, Provider, Tag};
use nakago_derive::Provider;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{domains::users::model::User, events::connections::Session};

use super::{
    connections::{Connections, CONNECTIONS},
    messages::IncomingMessage,
    messages::{
        IncomingMessage::Ping,
        OutgoingMessage::{Error, Pong},
    },
};

/// The socket::Handler Tag
pub const HANDLER: Tag<Handler> = Tag::new("socket::Handler");

/// WebSocket Event Handler
#[derive(Clone, new)]
pub struct Handler {
    connections: Arc<Connections>,
}

impl Handler {
    /// Handle `WebSocket` connections by setting up a message handler that deserializes them and
    /// determines how to handle
    pub async fn handle(&self, socket: WebSocket, user: Option<User>) {
        let (mut ws_write, mut ws_read) = socket.split();

        let (tx, rx) = mpsc::unbounded_channel();
        let mut rx = UnboundedReceiverStream::new(rx);

        tokio::task::spawn(async move {
            while let Some(message) = rx.next().await {
                ws_write
                    .send(message)
                    .unwrap_or_else(|err| {
                        eprintln!("websocket send error: {err}");
                    })
                    .await;
            }
        });

        let conn_id = self.connections.insert(tx, Session::new(user)).await;

        while let Some(result) = ws_read.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(err) => {
                    eprintln!("websocket error(uid={conn_id}): {err}");
                    break;
                }
            };

            match IncomingMessage::from_message(msg) {
                Ok(Some(message)) => {
                    if let Err(err) = self.route_message(&conn_id, message).await {
                        self.connections
                            .send(
                                &conn_id,
                                Error {
                                    message: err.to_string(),
                                }
                                .into(),
                            )
                            .await;

                        error!("{err}");
                    }
                }
                Ok(None) => {
                    // pass
                }
                Err(err) => {
                    eprintln!("json error(uid={conn_id}): {err}");
                }
            }
        }

        eprintln!("good bye user: {}", conn_id);

        self.connections.remove(&conn_id).await;
    }

    /// Route `WebSocket` messages to handlers
    pub async fn route_message(
        &self,
        conn_id: &str,
        message: IncomingMessage,
    ) -> anyhow::Result<()> {
        match message {
            Ping => self.handle_ping(conn_id).await,
        }
    }

    async fn handle_ping(&self, conn_id: &str) -> Result<()> {
        self.connections.send(conn_id, Pong.into()).await;

        Ok(())
    }
}

/// Provide a new WebSocket Event Handler
///
/// **Provides:** `events::SocketHandler`
///
/// **Depends on:**
///   - `Tag(events::Connections)`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Handler> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Handler>> {
        let connections = i.get(&CONNECTIONS).await?;

        Ok(Arc::new(Handler::new(connections)))
    }
}
