use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use derive_new::new;
use futures::{SinkExt, StreamExt, TryFutureExt};
use nakago::{provider, Inject, Provider, Tag};
use nakago_derive::Provider;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::connections::{Connections, Session};

/// A Router routes Websocket messages to the appropriate handler
#[async_trait]
pub trait Router: Send + Sync + Any {
    /// Route the given message to the appropriate handler
    async fn route(&self, msg: Message) -> anyhow::Result<()>;
}

/// WebSocket Event Handler
#[derive(Clone, new)]
pub struct Handler<U> {
    connections: Arc<Connections<U>>,
    router: Arc<Box<dyn Router>>,
}

impl<U: Clone> Handler<U> {
    /// Handle `WebSocket` connections by setting up a message handler that deserializes them and
    /// determines how to handle
    pub async fn handle(&self, socket: WebSocket, user: Option<U>) {
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

            if let Err(err) = self.router.route(msg).await {
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
    router_tag: Option<&'static Tag<Box<dyn Router>>>,
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

    /// Set a Tag for the Connections instance this Provider requires
    pub fn with_router_tag(self, router_tag: &'static Tag<Box<dyn Router>>) -> Self {
        Self {
            router_tag: Some(router_tag),
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

        let router = if let Some(tag) = self.router_tag {
            i.get(tag).await?
        } else {
            i.get_type::<Box<dyn Router>>().await?
        };

        Ok(Arc::new(Handler::new(connections, router)))
    }
}
