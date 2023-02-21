use async_trait::async_trait;

use super::{Inject, Result};

/// A trait for async injection Initializers
#[async_trait]
pub trait Initializer: Send + Sync {
    /// Operate on dependencies
    async fn init(&self, i: &mut Inject) -> Result<()>;
}

impl Inject {
    /// Use Initializer functions to operate on dependencies
    pub async fn init<I>(&mut self, initializers: Vec<I>) -> Result<()>
    where
        I: Initializer,
    {
        for initializer in initializers {
            initializer.init(self).await?;
        }

        Ok(())
    }
}
