use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::{IntoResponse, Response},
};
use biscuit::Empty;
use derive_new::new;
use futures::{SinkExt, StreamExt, TryFutureExt};
use http::StatusCode;
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
    async fn get_session(&self, token: Token<T>) -> anyhow::Result<Session>;
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
        let session = self.handler.get_session(token).await.map_err(Error)?;

        Ok(ws.on_upgrade(|socket| async move { self.handle(socket, session).await }))
    }

    /// Handle `WebSocket` connections by setting up a message handler that deserializes them and
    /// determines how to handle
    async fn handle(&self, socket: WebSocket, session: Session) {
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

        let conn_id = self.connections.insert(tx, session).await;

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

struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

/// Provide a new WebSocket Event Controller
#[derive(Default, new)]
pub struct Provide<Session: Any> {
    connections_tag: Option<&'static Tag<Connections<Session>>>,
    handler_tag: Option<&'static Tag<Box<dyn Handler<Session>>>>,
}

impl<Session: Any> Provide<Session> {
    /// Set a Tag for the Connections instance this Provider requires
    pub fn with_connections_tag(self, connections_tag: &'static Tag<Connections<Session>>) -> Self {
        Self {
            connections_tag: Some(connections_tag),
            ..self
        }
    }

    /// Set a Tag for the Handler instance this Provider requires
    pub fn with_handler_tag(self, handler_tag: &'static Tag<Box<dyn Handler<Session>>>) -> Self {
        Self {
            handler_tag: Some(handler_tag),
            ..self
        }
    }
}

#[Provider]
#[async_trait]
impl<Session> Provider<Controller<Session>> for Provide<Session>
where
    Session: Send + Sync + Any,
{
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Controller<Session>>> {
        let connections = if let Some(tag) = self.connections_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<Connections<Session>>().await?
        };

        let handler = if let Some(tag) = self.handler_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<Box<dyn Handler<Session>>>().await?
        };

        Ok(Arc::new(Controller::new(connections, handler)))
    }
}
