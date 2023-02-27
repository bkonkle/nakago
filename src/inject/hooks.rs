use async_trait::async_trait;
use std::any::Any;

use super::{Inject, Result};

/// A hook that can be run at various points in the lifecycle of an application
#[async_trait]
pub trait Hook: Any + Send {
    /// Handle the event by operating on the Inject container
    async fn handle(&mut self, i: &mut Inject) -> Result<()>;
}

/// A no-op hook that does nothing, for use as a default
pub struct NoOpHook {}

#[async_trait]
impl Hook for NoOpHook {
    async fn handle(&mut self, _i: &mut Inject) -> Result<()> {
        Ok(())
    }
}
