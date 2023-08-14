use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};
use std::sync::Arc;

use super::{Connections, SocketHandler};

/// The Connections Tag
pub const CONNECTIONS: Tag<Connections> = Tag::new("Connections");

/// Provide the default Connections implementation
///
/// **Provides:** `Arc<Connections>`
#[derive(Default)]
pub struct ProvideConnections {}

#[async_trait]
impl Provider for ProvideConnections {
    async fn provide(self: Arc<Self>, _i: Inject) -> InjectResult<Arc<Dependency>> {
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
impl Provider for ProvideSocket {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let connections = i.get(&CONNECTIONS).await?;

        Ok(Arc::new(SocketHandler::new(connections)))
    }
}
