use async_trait::async_trait;
use std::any::Any;

use super::{Inject, Result};

/// A hook that can be run at various points in the lifecycle of an application
#[async_trait]
pub trait Hook: Any + Send {
    /// Handle the event by operating on the Inject container
    async fn handle(&self, i: &'static mut Inject) -> Result<()>;
}

impl Inject {
    /// Handle a hook by running it against the Inject container
    pub async fn handle<H>(&'static mut self, hook: H) -> Result<()>
    where
        H: Hook,
    {
        hook.handle(self).await
    }
}
