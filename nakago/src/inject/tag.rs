use std::{any::Any, fmt::Display, marker::PhantomData, ops::Deref, sync::Arc};

use super::{Inject, Key, Provider, Result};

/// A dependency injection Tag representing a specific type
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tag<T> {
    tag: &'static str,
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
    /// Retrieve a reference to a tagged dependency if it exists, and return an error otherwise
    pub async fn get<T: Any + Send + Sync>(&self, tag: &'static Tag<T>) -> Result<Arc<T>> {
        self.get_key(Key::from_tag::<T>(tag.tag)).await
    }

    /// Retrieve a reference to a tagged dependency if it exists in the map
    pub async fn get_opt<T: Any + Send + Sync>(
        &self,
        tag: &'static Tag<T>,
    ) -> Result<Option<Arc<T>>> {
        self.get_key_opt(Key::from_tag::<T>(tag.tag)).await
    }

    /// Provide a tagged dependency directly
    pub async fn inject<T: Any + Sync + Send>(&self, tag: &'static Tag<T>, dep: T) -> Result<()> {
        self.inject_key(Key::from_tag::<T>(tag.tag), dep).await
    }

    /// Replace an existing tagged dependency directly
    pub async fn replace<T: Any + Sync + Send>(&self, tag: &'static Tag<T>, dep: T) -> Result<()> {
        self.replace_key(Key::from_tag::<T>(tag.tag), dep).await
    }

    /// Register a Provider for a tagged dependency
    pub async fn provide<T: Any + Sync + Send>(
        &self,
        tag: &'static Tag<T>,
        provider: impl Provider + 'static,
    ) -> Result<()> {
        self.provide_key::<T>(Key::from_tag::<T>(tag.tag), provider)
            .await
    }

    /// Replace an existing Provider for a tagged dependency
    pub async fn replace_with<T: Any + Sync + Send>(
        &self,
        tag: &'static Tag<T>,
        provider: impl Provider + 'static,
    ) -> Result<()> {
        self.replace_key_with::<T>(Key::from_tag::<T>(tag.tag), provider)
            .await
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;

    use super::*;
    use crate::inject::{
        container::test::{HasId, OtherService, TestService},
        Result,
    };

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
}
