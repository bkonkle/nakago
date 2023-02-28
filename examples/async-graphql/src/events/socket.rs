use anyhow::Result;
use axum::extract::ws::WebSocket;
use futures::{SinkExt, StreamExt, TryFutureExt};
use log::error;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::{
    connections::Connections,
    messages::IncomingMessage,
    messages::{
        IncomingMessage::Ping,
        OutgoingMessage::{Error, Pong},
    },
};

/// WebSocket Event Handler
#[derive(Clone)]
pub struct SocketHandler {
    connections: Arc<Connections>,
}

impl SocketHandler {
    /// Create a new Event Handler instance with dependencies
    pub fn new(connections: Arc<Connections>) -> Self {
        Self { connections }
    }

    /// Handle `WebSocket` connections by setting up a message handler that deserializes them and
    /// determines how to handle
    pub async fn handle(&self, socket: WebSocket) {
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

        let conn_id = self.connections.insert(tx).await;

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
