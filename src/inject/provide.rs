use async_trait::async_trait;
use std::any::Any;

use super::{tag::Tag, Inject, Result};

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<T>
where
    T: Any + Send + Sync + ?Sized,
{
    /// Provide a dependency for the container
    async fn provide(&self, i: &Inject) -> Result<Box<T>>;
}

impl Inject {
    /// Use a Provider function to inject a dependency
    pub async fn provide<T, P>(&mut self, provider: P) -> Result<()>
    where
        T: Any + Sync + Send + ?Sized,
        P: Provider<T>,
    {
        self.inject::<T>(provider.provide(self).await?)
    }

    /// Use a Provider function to replace an existing dependency
    pub async fn replace_with<T, P>(&mut self, provider: P) -> Result<()>
    where
        T: Any + Sync + Send + ?Sized,
        P: Provider<T>,
    {
        self.replace::<T>(provider.provide(self).await?)
    }

    /// Use a Provider function to inject a tagged dependency
    pub async fn provide_tag<T, P>(&mut self, provider: P, tag: &'static Tag<T>) -> Result<()>
    where
        T: Any + Sync + Send + ?Sized,
        P: Provider<T>,
    {
        self.inject_tag::<T>(provider.provide(self).await?, tag)
    }

    /// Use a Provider function to replace a tagged dependency
    pub async fn replace_tag_with<T, P>(&mut self, provider: P, tag: &'static Tag<T>) -> Result<()>
    where
        T: Any + Sync + Send + ?Sized,
        P: Provider<T>,
    {
        self.replace_tag::<T>(provider.provide(self).await?, tag)
    }
}

#[cfg(test)]
mod test {
    use std::{any::type_name, sync::Arc};

    use fake::Fake;

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
        async fn provide(&self, _i: &Inject) -> Result<Box<TestService>> {
            Ok(Box::new(TestService::new(self.id.clone())))
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
        async fn provide(&self, _i: &Inject) -> Result<Box<OtherService>> {
            Ok(Box::new(OtherService::new(self.id.clone())))
        }
    }

    #[async_trait]
    impl Provider<Arc<dyn HasId>> for OtherServiceProvider {
        async fn provide(&self, _i: &Inject) -> Result<Box<Arc<dyn HasId>>> {
            Ok(Box::new(Arc::new(OtherService::new(self.id.clone()))))
        }
    }

    #[derive(Default)]
    pub struct TestServiceHasIdProvider {}

    #[async_trait]
    impl Provider<Arc<dyn HasId>> for TestServiceHasIdProvider {
        async fn provide(&self, _i: &Inject) -> Result<Box<Arc<dyn HasId>>> {
            Ok(Box::new(Arc::new(OtherService::new(
                "test-service".to_string(),
            ))))
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
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
            &SERVICE_TAG,
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

        i.provide_tag(TestServiceHasIdProvider::default(), &DYN_TAG)
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
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
            &SERVICE_TAG,
        )
        .await?;

        let result = i
            .provide_tag(
                TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
                &SERVICE_TAG,
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
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
            &SERVICE_TAG,
        )
        .await?;

        // Override the instance that was injected the first time
        i.replace_tag_with(TestServiceProvider::new(expected.clone()), &SERVICE_TAG)
            .await?;

        let result = i.get_tag::<TestService>(&SERVICE_TAG)?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_tag_with_not_found() -> Result<()> {
        let mut i = Inject::default();

        i.provide_tag(
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
            &SERVICE_TAG,
        )
        .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace_tag_with(
                OtherServiceProvider::new(fake::uuid::UUIDv4.fake()),
                &OTHER_TAG,
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
