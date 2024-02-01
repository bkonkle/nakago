use std::any::Any;

use async_trait::async_trait;
use axum::extract::ws::Message;
use nakago_axum::auth::Subject;

/// A Controller routes Websocket messages to the appropriate handler
#[async_trait]
pub trait Controller<U>: Send + Sync + Any {
    /// Route the given message to the appropriate handler
    async fn route(&self, conn_id: &str, msg: Message) -> anyhow::Result<()>;

    /// Get the User from the Subject
    async fn get_user(&self, sub: Subject) -> Option<U>;
}
