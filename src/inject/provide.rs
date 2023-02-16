use async_trait::async_trait;
use std::any::Any;

use super::{tag::Tag, Inject, Result};

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<T>
where
    T: Any + Send + Sync,
{
    /// Provide a dependency for the container
    async fn provide(&self, i: &Inject) -> Result<T>;
}

impl Inject {
    /// Use a Provider function to inject a dependency
    pub async fn provide<T, P>(&mut self, provider: P) -> Result<()>
    where
        T: Any + Sync + Send,
        P: Provider<T>,
    {
        self.inject::<T>(provider.provide(self).await?)
    }

    /// Use a Provider function to replace an existing dependency
    pub async fn replace_with<T, P>(&mut self, provider: P) -> Result<()>
    where
        T: Any + Sync + Send,
        P: Provider<T>,
    {
        self.replace::<T>(provider.provide(self).await?)
    }

    /// Use a Provider function to inject a tagged dependency
    pub async fn provide_tag<T, P>(&mut self, tag: &'static Tag<T>, provider: P) -> Result<()>
    where
        T: Any + Sync + Send,
        P: Provider<T>,
    {
        self.inject_tag::<T>(tag, provider.provide(self).await?)
    }

    /// Use a Provider function to replace a tagged dependency
    pub async fn replace_tag_with<T, P>(&mut self, tag: &'static Tag<T>, provider: P) -> Result<()>
    where
        T: Any + Sync + Send,
        P: Provider<T>,
    {
        self.replace_tag::<T>(tag, provider.provide(self).await?)
    }
}

#[cfg(test)]
mod test {
    use fake::Fake;
    use std::{any::type_name, sync::Arc};
    use tokio::time::{sleep, Duration};

    use crate::inject::{
        container::test::{HasId, OtherService, TestService},
        tag::test::{DYN_TAG, OTHER_TAG, SERVICE_TAG},
        Key,
    };

    use super::*;

    pub struct TestServiceProvider {
        id: String,
    }

    impl TestServiceProvider {
        /// Create a new TestServiceProvider instance
        pub fn new(id: String) -> Self {
            Self { id }
        }
    }

    #[async_trait]
    impl Provider<TestService> for TestServiceProvider {
        async fn provide(&self, _i: &Inject) -> Result<TestService> {
            Ok(TestService::new(self.id.clone()))
        }
    }

    pub struct OtherServiceProvider {
        id: String,
    }

    impl OtherServiceProvider {
        /// Create a new OtherServiceProvider instance
        pub fn new(id: String) -> Self {
            Self { id }
        }
    }

    #[async_trait]
    impl Provider<OtherService> for OtherServiceProvider {
        async fn provide(&self, _i: &Inject) -> Result<OtherService> {
            Ok(OtherService::new(self.id.clone()))
        }
    }

    #[async_trait]
    impl Provider<Arc<dyn HasId>> for OtherServiceProvider {
        async fn provide(&self, _i: &Inject) -> Result<Arc<dyn HasId>> {
            Ok(Arc::new(OtherService::new(self.id.clone())))
        }
    }

    #[derive(Default)]
    pub struct TestServiceHasIdProvider {}

    #[async_trait]
    impl Provider<Arc<dyn HasId>> for TestServiceHasIdProvider {
        async fn provide(&self, i: &Inject) -> Result<Arc<dyn HasId>> {
            // Trigger a borrow so that the reference to `Inject` has to be held across the await
            // point below, to test issues with Inject thread safety.
            let _ = i.get::<String>();

            sleep(Duration::from_millis(1)).await;

            Ok(Arc::new(OtherService::new("test-service".to_string())))
        }
    }

    #[tokio::test]
    async fn test_provide_success() -> Result<()> {
        let mut i = Inject::default();

        i.provide(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        assert!(
            i.0.contains_key(&Key::from_type_id::<TestService>()),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_dyn_success() -> Result<()> {
        let mut i = Inject::default();

        i.provide(TestServiceHasIdProvider::default()).await?;

        assert!(
            i.0.contains_key(&Key::from_type_id::<Arc<dyn HasId>>()),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_occupied() -> Result<()> {
        let mut i = Inject::default();

        i.provide(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        let result = i
            .provide(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await;

        if let Err(err) = result {
            assert_eq!(
                format!("{} has already been provided", type_name::<TestService>()),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_with_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override the instance that was injected the first time
        i.replace_with(TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.get::<TestService>()?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_with_not_found() -> Result<()> {
        let mut i = Inject::default();

        i.provide(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace_with::<OtherService, _>(OtherServiceProvider::new(fake::uuid::UUIDv4.fake()))
            .await;

        if let Err(err) = result {
            assert_eq!(
                format!(
                    "{} was not found\n\nAvailable:\n - {}",
                    type_name::<OtherService>(),
                    type_name::<TestService>()
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_tag_success() -> Result<()> {
        let mut i = Inject::default();

        i.provide_tag(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        assert!(
            i.0.contains_key(&Key::from_tag::<TestService>(&SERVICE_TAG)),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_tag_dyn_success() -> Result<()> {
        let mut i = Inject::default();

        i.provide_tag(&DYN_TAG, TestServiceHasIdProvider::default())
            .await?;

        assert!(
            i.0.contains_key(&Key::from_tag::<Arc<dyn HasId>>(&DYN_TAG)),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_tag_occupied() -> Result<()> {
        let mut i = Inject::default();

        i.provide_tag(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        let result = i
            .provide_tag(
                &SERVICE_TAG,
                TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
            )
            .await;

        if let Err(err) = result {
            assert_eq!(
                format!("{} has already been provided", SERVICE_TAG),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_tag_with_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide_tag(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        // Override the instance that was injected the first time
        i.replace_tag_with(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.get_tag::<TestService>(&SERVICE_TAG)?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_tag_with_not_found() -> Result<()> {
        let mut i = Inject::default();

        i.provide_tag(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace_tag_with(
                &OTHER_TAG,
                OtherServiceProvider::new(fake::uuid::UUIDv4.fake()),
            )
            .await;

        if let Err(err) = result {
            assert_eq!(
                format!(
                    "{} was not found\n\nAvailable:\n - {}",
                    OTHER_TAG, SERVICE_TAG
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }
}
