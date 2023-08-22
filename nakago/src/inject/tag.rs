use std::{any::Any, fmt::Display, marker::PhantomData, ops::Deref, sync::Arc};

use crate::Dependency;

use super::{Inject, Key, Provider, Result};

/// A dependency injection Tag representing a specific type
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tag<T: ?Sized> {
    pub(crate) tag: &'static str,
    _phantom: fn() -> PhantomData<T>,
}

impl<T> Tag<T>
where
    T: Sync + Send,
{
    /// Create a new Tag instance
    pub const fn new(tag: &'static str) -> Self {
        Self {
            tag,
            _phantom: PhantomData::default,
        }
    }
}

impl<T> Display for Tag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tag({})", self.tag)
    }
}

impl<T> Deref for Tag<T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.tag
    }
}

impl Inject {
    /// Retrieve a reference to a Tagged Dependency if it exists. Return a NotFound error if the Tag
    /// isn't present.
    pub async fn get<T: Any + Send + Sync>(&self, tag: &'static Tag<T>) -> Result<Arc<T>> {
        self.get_key(Key::from_tag(tag)).await
    }

    /// Retrieve a reference to a Tagged Dependency if it exists.
    pub async fn get_opt<T: Any + Send + Sync>(
        &self,
        tag: &'static Tag<T>,
    ) -> Result<Option<Arc<T>>> {
        self.get_key_opt(Key::from_tag(tag)).await
    }

    /// Override an existing Tagged Dependency directly, using core::future::ready to wrap it in an
    /// immediately resolving Pending Future. Return true if the Key was already present.
    pub async fn override_tag<T: Any + Send + Sync>(
        &self,
        tag: &'static Tag<T>,
        dep: T,
    ) -> Result<bool> {
        self.override_key(Key::from_tag(tag), dep).await
    }

    /// Provide a Tagged Dependency directly, using core::future::ready to wrap it in an immediately
    /// resolving Pending Future.
    pub async fn inject<T: Any + Sync + Send>(&self, tag: &'static Tag<T>, dep: T) -> Result<()> {
        self.inject_key(Key::from_tag(tag), dep).await
    }

    /// Replace an existing Tagged Dependency directly, using core::future::ready to wrap it in an
    /// immediately resolving Pending Future. Return a NotFound error if the Key isn't present.
    pub async fn replace<T: Any + Sync + Send>(&self, tag: &'static Tag<T>, dep: T) -> Result<()> {
        self.replace_key(Key::from_tag(tag), dep).await
    }

    /// Inject a Dependency Provider for a Tag
    pub async fn provide<T: Any + Sync + Send>(
        &self,
        tag: &'static Tag<T>,
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Result<()> {
        self.provide_key::<T>(Key::from_tag(tag), provider).await
    }

    /// Inject a replacement Dependency Provider if the Tag is present
    pub async fn replace_with<T: Any + Sync + Send>(
        &self,
        tag: &'static Tag<T>,
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Result<()> {
        self.replace_key_with::<T>(Key::from_tag(tag), provider)
            .await
    }

    /// Remove a Tagged Dependency from the container and try to unwrap it from the Arc, which will
    /// only succeed if there are no other strong pointers to the value. Any Arcs handed out will
    /// still be valid, but the container will no longer hold a reference. Return a NotFound error
    /// if the Tag isn't present.
    pub async fn consume<T: Any + Send + Sync>(&self, tag: &'static Tag<T>) -> Result<T> {
        self.consume_key(Key::from_tag(tag)).await
    }

    /// Remove a Tagged Dependency from the container and try to unwrap it from the Arc, which will
    /// only succeed if there are no other strong pointers to the value. Any Arcs handed out will
    /// still be valid, but the container will no longer hold a reference.
    pub async fn consume_opt<T: Any + Send + Sync>(
        &self,
        tag: &'static Tag<T>,
    ) -> Result<Option<T>> {
        self.consume_key_opt(Key::from_tag(tag)).await
    }

    /// Discard a Tagged Dependency from the container. Any Arcs handed out will still be valid, but
    /// the container will no longer hold a reference.
    pub async fn remove<T: Any + Send + Sync>(&self, tag: &'static Tag<T>) -> Result<()> {
        self.remove_key(Key::from_tag(tag)).await
    }

    /// Destroy the container and discard all Dependencies except for the given Tag. Any Arcs handed
    /// out will still be valid, but the container will be fully unloaded and all references will be
    /// dropped. Return a NotFound error if the Key isn't present.
    pub async fn eject<T: Any + Send + Sync>(self, tag: &'static Tag<T>) -> Result<T> {
        self.eject_key(Key::from_tag(tag)).await
    }

    /// Destroy the container and discard all Dependencies except for the given Tag. Any Arcs handed
    /// out will still be valid, but the container will be fully unloaded and all references will be
    /// dropped.
    pub async fn eject_opt<T: Any + Send + Sync>(self, tag: &'static Tag<T>) -> Result<Option<T>> {
        self.eject_key_opt(Key::from_tag(tag)).await
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;

    use crate::inject::{
        container::test::{
            HasId, HasIdProvider, OtherService, OtherServiceProvider, TestService,
            TestServiceProvider,
        },
        Result,
    };

    use super::*;

    pub const SERVICE_TAG: Tag<TestService> = Tag::new("InMemoryTestService");
    pub const OTHER_TAG: Tag<OtherService> = Tag::new("InMemoryOtherService");
    pub const DYN_TAG: Tag<Box<dyn HasId>> = Tag::new("DynHasIdService");

    trait DynamicService: Sync + Send {
        fn test_fn(&self) {}
    }

    #[tokio::test]
    async fn test_inject_tag_success() -> Result<()> {
        let i = Inject::default();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_tag::<TestService>(&SERVICE_TAG)),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_inject_tag_occupied() -> Result<()> {
        let i = Inject::default();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Inject the same type a second time
        let result = i
            .inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))
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
    async fn test_get_tag_opt_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(&SERVICE_TAG, TestService::new(expected.clone()))
            .await?;

        let result = i.get_opt(&SERVICE_TAG).await?.unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_tag_opt_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_opt(&SERVICE_TAG).await?;

        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_tag_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(&SERVICE_TAG, TestService::new(expected.clone()))
            .await?;

        let result = i.get(&SERVICE_TAG).await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_dyn_tag_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject::<Box<dyn HasId>>(&DYN_TAG, Box::new(TestService::new(expected.clone())))
            .await?;

        let result = i.get(&DYN_TAG).await?;

        assert_eq!(expected, result.get_id());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_tag_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get(&SERVICE_TAG).await;

        if let Err(err) = result {
            assert_eq!(
                format!("{} was not found\n\nAvailable: (empty)", SERVICE_TAG),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_tag_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override the instance that was injected the first time
        i.replace(&SERVICE_TAG, TestService::new(expected.clone()))
            .await?;

        let result = i.get(&SERVICE_TAG).await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_not_found() -> Result<()> {
        let i = Inject::default();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))
            .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace(&OTHER_TAG, OtherService::new(fake::uuid::UUIDv4.fake()))
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

    #[tokio::test]
    async fn test_provide_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_tag::<TestService>(&SERVICE_TAG)),
            "key does not exist in injection container"
        );

        let result = i.get(&SERVICE_TAG).await?;

        assert_eq!(result.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_multiple() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        i.provide(
            &OTHER_TAG,
            OtherServiceProvider::new(expected_other.clone()),
        )
        .await?;

        let result = i.get(&SERVICE_TAG).await?;
        let other = i.get(&OTHER_TAG).await?;

        assert_eq!(result.id, expected);
        assert_eq!(other.other_id, expected_other);

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_dyn_success() -> Result<()> {
        let i = Inject::default();

        i.provide(&DYN_TAG, HasIdProvider::default()).await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_tag::<Box<dyn HasId>>(&DYN_TAG)),
            "key does not exist in injection container"
        );

        let result = i.get(&DYN_TAG).await?;

        assert_eq!(result.get_id(), "test-service");

        Ok(())
    }

    #[tokio::test]
    async fn test_provide_occupied() -> Result<()> {
        let i = Inject::default();

        let expected = format!("{} has already been provided", SERVICE_TAG);

        i.provide(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        let result = i
            .provide(
                &SERVICE_TAG,
                TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
            )
            .await;

        if let Err(err) = result {
            assert_eq!(err.to_string(), expected);
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_with_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        // Retrieve the dependency once to trigger the Provider
        i.get(&SERVICE_TAG).await?;

        // Override the instance that was injected the first time
        i.replace_with(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.get(&SERVICE_TAG).await?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_with_multiple() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        i.provide(
            &OTHER_TAG,
            OtherServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        // Retrieve the dependencies once to trigger the Providers
        i.get(&SERVICE_TAG).await?;
        i.get(&OTHER_TAG).await?;

        // Override the instances that were injected the first time
        i.replace_with(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        i.replace_with(
            &OTHER_TAG,
            OtherServiceProvider::new(expected_other.clone()),
        )
        .await?;

        let result = i.get(&SERVICE_TAG).await?;
        let other = i.get(&OTHER_TAG).await?;

        assert_eq!(result.id, expected);
        assert_eq!(other.other_id, expected_other);

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_with_not_found() -> Result<()> {
        let i = Inject::default();

        let expected = format!(
            "{} was not found\n\nAvailable:\n - {}",
            OTHER_TAG, SERVICE_TAG
        );

        i.provide(
            &SERVICE_TAG,
            TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
        )
        .await?;

        // Override a type that doesn't have any instances yet
        let result = i
            .replace_with(
                &OTHER_TAG,
                OtherServiceProvider::new(fake::uuid::UUIDv4.fake()),
            )
            .await;

        if let Err(err) = result {
            assert_eq!(err.to_string(), expected);
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_provider_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        let result = i.consume(&SERVICE_TAG).await?;

        assert_eq!(result.id, expected);

        assert!(
            !i.0.read().await.contains_key(&Key::from_tag(&SERVICE_TAG)),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_provider_multiple() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();
        let expected_other: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        i.provide(
            &OTHER_TAG,
            OtherServiceProvider::new(expected_other.clone()),
        )
        .await?;

        let result = i.consume(&SERVICE_TAG).await?;
        let other = i.consume(&OTHER_TAG).await?;

        assert_eq!(result.id, expected);
        assert_eq!(other.other_id, expected_other);

        assert!(
            !i.0.read().await.contains_key(&Key::from_tag(&SERVICE_TAG)),
            "key still exists in injection container"
        );

        assert!(
            !i.0.read().await.contains_key(&Key::from_tag(&OTHER_TAG)),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_consume_pending_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        // Trigger the Provider so that the value is Pending below
        i.get(&SERVICE_TAG).await?;

        let result = i.consume(&SERVICE_TAG).await?;

        assert_eq!(result.id, expected);

        assert!(
            !i.0.read().await.contains_key(&Key::from_tag(&SERVICE_TAG)),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_provider_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        i.remove(&SERVICE_TAG).await?;

        assert!(
            !i.0.read().await.contains_key(&Key::from_tag(&SERVICE_TAG)),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_pending_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        // Trigger the Provider so that the value is Pending below
        i.get(&SERVICE_TAG).await?;

        i.remove(&SERVICE_TAG).await?;

        assert!(
            !i.0.read().await.contains_key(&Key::from_tag(&SERVICE_TAG)),
            "key still exists in injection container"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_eject_pending_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(&SERVICE_TAG, TestService::new(expected.clone()))
            .await?;

        let service = i.eject(&SERVICE_TAG).await?;

        // This is commented out because it demonstrates a case prevented by the compiler - usage
        // of the container after ejection.
        //
        // i.get(&SERVICE_TAG).await?;

        assert_eq!(service.id, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_eject_provider_success() -> Result<()> {
        let i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.provide(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
            .await?;

        let service = i.eject(&SERVICE_TAG).await?;

        assert_eq!(service.id, expected);

        Ok(())
    }
}
