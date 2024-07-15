use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use thiserror::Error;

use super::{errors, from_hook_error, provider, Inject};

/// A hook that can be run at various points in the lifecycle of an application
#[async_trait]
pub trait Hook: Any + Send {
    /// Handle the event by operating on the Inject container
    async fn handle(&self, i: Inject) -> Result<()>;
}

impl Inject {
    /// Handle a hook by running it against the Inject container
    pub async fn handle<H>(&self, hook: H) -> errors::Result<()>
    where
        H: Hook,
    {
        hook.handle(self.clone()).await.map_err(from_hook_error)
    }
}

/// A Hook Result
pub type Result<T> = std::result::Result<T, Error>;

/// Hook Errors
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// A generic error thrown from a Hook
    #[error("hook failure")]
    Any(#[from] Arc<anyhow::Error>),

    /// An injection error thrown from a Hook
    #[error("injection failure")]
    Inject(#[from] errors::Error),

    /// A Provider error thrown from a Hook
    #[error("provider failure")]
    Provider(#[from] provider::Error),
}

/// Wrap an error that can be converted into an Anyhow error with a Hook error
pub fn to_hook_error<E>(e: E) -> Error
where
    anyhow::Error: From<E>,
{
    Error::Any(Arc::new(e.into()))
}

#[cfg(test)]
mod tests {
    use fake::Fake;

    use super::*;
    use crate::{
        inject::{container::test::TestService, tag::test::SERVICE_TAG, Key},
        Inject,
    };

    #[derive(Debug)]
    struct TestHook;

    #[async_trait]
    impl Hook for TestHook {
        async fn handle(&self, i: Inject) -> Result<()> {
            i.inject_tag(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))
                .await?;

            Ok(())
        }
    }

    #[tokio::test]
    async fn test_hook() {
        let i = Inject::default();

        let hook = TestHook;

        i.handle(hook).await.unwrap();

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_tag::<TestService>(&SERVICE_TAG)),
            "key does not exist in injection container"
        );
    }
}
