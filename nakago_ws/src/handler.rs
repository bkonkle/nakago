use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use derive_new::new;
use futures::{SinkExt, StreamExt, TryFutureExt};
use nakago::{provider, Inject, Provider, Tag};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::{connections::Session, Connections, Controller};

/// WebSocket Event Handler
#[derive(Clone, new)]
pub struct Handler<U> {
    connections: Arc<Connections<U>>,
    controller: Arc<Box<dyn Controller<U>>>,
}

impl<U: Send + Sync + Clone + Any> Handler<U> {
    /// Handle requests for new WebSocket connections
    pub async fn upgrade(
        self: Arc<Self>,
        sub: Subject,
        ws: WebSocketUpgrade,
    ) -> axum::response::Result<impl IntoResponse> {
        // Retrieve the request User, if username is present
        let user = self.controller.get_user(sub).await;

        Ok(ws.on_upgrade(|socket| async move { self.handle(socket, user).await }))
    }

    /// Handle `WebSocket` connections by setting up a message handler that deserializes them and
    /// determines how to handle
    async fn handle(&self, socket: WebSocket, user: Option<U>) {
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

            if let Err(err) = self.controller.route(&conn_id, msg).await {
                eprintln!("json error(uid={conn_id}): {err}");
                break;
            }
        }

        eprintln!("good bye user: {}", conn_id);

        self.connections.remove(&conn_id).await;
    }
}

/// Provide a new WebSocket Event Handler
#[derive(Default, new)]
pub struct Provide<U: Any> {
    connections_tag: Option<&'static Tag<Connections<U>>>,
    controller_tag: Option<&'static Tag<Box<dyn Controller<U>>>>,
    _phantom: std::marker::PhantomData<U>,
}

impl<U: Any> Provide<U> {
    /// Set a Tag for the Connections instance this Provider requires
    pub fn with_connections_tag(self, connections_tag: &'static Tag<Connections<U>>) -> Self {
        Self {
            connections_tag: Some(connections_tag),
            ..self
        }
    }

    /// Set a Tag for the Controller instance this Provider requires
    pub fn with_controller_tag(self, controller_tag: &'static Tag<Box<dyn Controller<U>>>) -> Self {
        Self {
            controller_tag: Some(controller_tag),
            ..self
        }
    }
}

#[Provider]
#[async_trait]
impl<U> Provider<Handler<U>> for Provide<U>
where
    U: Send + Sync + Any,
{
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Handler<U>>> {
        let connections = if let Some(tag) = self.connections_tag {
            i.get(tag).await?
        } else {
            i.get_type::<Connections<U>>().await?
        };

        let controller = if let Some(tag) = self.controller_tag {
            i.get(tag).await?
        } else {
            i.get_type::<Box<dyn Controller<U>>>().await?
        };

        Ok(Arc::new(Handler::new(connections, controller)))
    }
}
