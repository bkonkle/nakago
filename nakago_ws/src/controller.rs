use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use biscuit::Empty;
use derive_new::new;
use futures::{SinkExt, StreamExt, TryFutureExt};
use mockall::automock;
use nakago::{provider, Inject, Provider, Tag};
use nakago_derive::Provider;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::auth::Token;

use super::Connections;

/// A Handler handles Websocket messages
#[automock]
#[async_trait]
pub trait Handler<Session, T = Empty>: Send + Sync + Any
where
    Session: Send + Sync + Any,
    T: Default + Send + Sync + Any,
{
    /// Route the given message to the appropriate handler
    async fn route(&self, conn_id: &str, msg: Message) -> anyhow::Result<()>;

    /// Get the User from the Subject
    async fn get_session(&self, token: Token<T>) -> Option<Session>;
}

/// WebSocket Controller
#[derive(Clone, new)]
pub struct Controller<Session> {
    connections: Arc<Connections<Session>>,
    handler: Arc<Box<dyn Handler<Session>>>,
}

impl<Session: Default + Send + Sync + Clone + Any> Controller<Session> {
    /// Handle requests for new WebSocket connections
    pub async fn upgrade(
        self: Arc<Self>,
        token: Token,
        ws: WebSocketUpgrade,
    ) -> axum::response::Result<impl IntoResponse> {
        // Retrieve the request Session
        let session = self.handler.get_session(token).await;

        Ok(ws.on_upgrade(|socket| async move { self.handle(socket, session).await }))
    }

    /// Handle `WebSocket` connections by setting up a message handler that deserializes them and
    /// determines how to handle
    async fn handle(&self, socket: WebSocket, session: Option<Session>) {
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

        let conn_id = self
            .connections
            .insert(tx, session.unwrap_or_default())
            .await;

        while let Some(result) = ws_read.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(err) => {
                    eprintln!("websocket error(uid={conn_id}): {err}");
                    break;
                }
            };

            if let Err(err) = self.handler.route(&conn_id, msg).await {
                eprintln!("json error(uid={conn_id}): {err}");
                break;
            }
        }

        eprintln!("good bye user: {}", conn_id);

        self.connections.remove(&conn_id).await;
    }
}

/// Provide a new WebSocket Event Controller
#[derive(Default, new)]
pub struct Provide<U: Any> {
    connections_tag: Option<&'static Tag<Connections<U>>>,
    handler_tag: Option<&'static Tag<Box<dyn Handler<U>>>>,
}

impl<U: Any> Provide<U> {
    /// Set a Tag for the Connections instance this Provider requires
    pub fn with_connections_tag(self, connections_tag: &'static Tag<Connections<U>>) -> Self {
        Self {
            connections_tag: Some(connections_tag),
            ..self
        }
    }

    /// Set a Tag for the Handler instance this Provider requires
    pub fn with_handler_tag(self, handler_tag: &'static Tag<Box<dyn Handler<U>>>) -> Self {
        Self {
            handler_tag: Some(handler_tag),
            ..self
        }
    }
}

#[Provider]
#[async_trait]
impl<U> Provider<Controller<U>> for Provide<U>
where
    U: Send + Sync + Any,
{
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Controller<U>>> {
        let connections = if let Some(tag) = self.connections_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<Connections<U>>().await?
        };

        let handler = if let Some(tag) = self.handler_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<Box<dyn Handler<U>>>().await?
        };

        Ok(Arc::new(Controller::new(connections, handler)))
    }
}
