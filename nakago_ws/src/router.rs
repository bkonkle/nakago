use std::any::Any;

use async_trait::async_trait;
use axum::extract::ws::Message;
use nakago::Tag;

/// Router
pub const ROUTER: Tag<Box<dyn Router>> = Tag::new("nakago_ws::Router");

/// A Router routes Websocket messages to the appropriate handler
#[async_trait]
pub trait Router: Send + Sync + Any {
    /// Route the given message to the appropriate handler
    async fn route(&self, conn_id: &str, msg: Message) -> anyhow::Result<()>;
}
