use async_trait::async_trait;
use nakago::{inject, Tag};
use std::sync::Arc;

use super::{Connections, SocketHandler};

/// The Connections Tag
pub const CONNECTIONS: Tag<Arc<Connections>> = Tag::new("Connections");

/// Provide the default Connections implementation
///
/// **Provides:** `Arc<Connections>`
#[derive(Default)]
pub struct ProvideConnections {}

#[async_trait]
impl inject::Provider<Arc<Connections>> for ProvideConnections {
    async fn provide(&self, _i: &inject::Inject) -> inject::Result<Arc<Connections>> {
        Ok(Arc::new(Connections::default()))
    }
}

/// The SocketHandler Tag
pub const SOCKET_HANDLER: Tag<SocketHandler> = Tag::new("SocketHandler");

/// Provide a new WebSocket Event Handler
///
/// **Provides:** `SocketHandler`
///
/// **Depends on:**
///   - `Tag(Connections)`
///   - `Tag(CommandsController)`
#[derive(Default)]
pub struct ProvideSocket {}

#[async_trait]
impl inject::Provider<SocketHandler> for ProvideSocket {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<SocketHandler> {
        let connections = i.get(&CONNECTIONS)?;

        Ok(SocketHandler::new(connections.clone()))
    }
}
