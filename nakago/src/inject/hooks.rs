use std::any::Any;

use async_trait::async_trait;

use super::{Inject, Result};

/// A hook that can be run at various points in the lifecycle of an application
#[async_trait]
pub trait Hook: Any + Send {
    /// Handle the event by operating on the Inject container
    async fn handle(&self, i: Inject) -> Result<()>;
}

impl Inject {
    /// Handle a hook by running it against the Inject container
    pub async fn handle<H>(&self, hook: H) -> Result<()>
    where
        H: Hook,
    {
        hook.handle(self.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;

    use super::*;
    use crate::{
        inject::{self, container::test::TestService, tag::test::SERVICE_TAG, Key},
        Inject,
    };

    #[derive(Debug)]
    struct TestHook;

    #[async_trait]
    impl Hook for TestHook {
        async fn handle(&self, i: Inject) -> inject::Result<()> {
            i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))
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
