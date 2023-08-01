use std::{any::Any, fmt::Display, marker::PhantomData, ops::Deref};

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
    pub fn get<T: Any + Send + Sync>(&self, tag: &'static Tag<T>) -> Result<&T> {
        self.get_key(Key::from_tag::<T>(tag.tag))
    }

    /// Retrieve a reference to a tagged dependency if it exists in the map
    pub fn get_opt<T: Any + Send + Sync>(&self, tag: &'static Tag<T>) -> Result<Option<&T>> {
        self.get_key_opt(Key::from_tag::<T>(tag.tag))
    }

    /// Provide a tagged dependency directly
    pub fn inject<T: Any + Sync + Send>(&mut self, tag: &'static Tag<T>, dep: T) -> Result<()> {
        self.inject_key(Key::from_tag::<T>(tag.tag), dep)
    }

    /// Replace an existing tagged dependency directly
    pub fn replace<T: Any + Sync + Send>(&mut self, tag: &'static Tag<T>, dep: T) -> Result<()> {
        self.replace_key(Key::from_tag::<T>(tag.tag), dep)
    }

    /// Register a Provider for a tagged dependency
    pub fn provide<T: Any + Sync + Send>(
        &mut self,
        tag: &'static Tag<T>,
        provider: Box<dyn Provider<Box<dyn Any + Send + Sync>>>,
    ) -> Result<()> {
        self.provide_key(Key::from_tag::<T>(tag.tag), provider)
    }

    /// Replace an existing Provider for a tagged dependency
    pub fn replace_provider<T: Any + Sync + Send>(
        &mut self,
        tag: &'static Tag<T>,
        provider: Box<dyn Provider<Box<dyn Any + Send + Sync>>>,
    ) -> Result<()> {
        self.replace_key_provider(Key::from_tag::<T>(tag.tag), provider)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use fake::Fake;

    use super::*;
    use crate::inject::{
        container::test::{HasId, OtherService, TestService},
        Result,
    };

    pub const SERVICE_TAG: Tag<TestService> = Tag::new("InMemoryTestService");
    pub const OTHER_TAG: Tag<OtherService> = Tag::new("InMemoryOtherService");
    pub const DYN_TAG: Tag<Arc<dyn HasId>> = Tag::new("DynHasIdService");

    trait DynamicService: Sync + Send {
        fn test_fn(&self) {}
    }

    #[test]
    fn test_inject_tag_success() -> Result<()> {
        let mut i = Inject::default();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))?;

        assert!(
            i.0.contains_key(&Key::from_tag::<TestService>(&SERVICE_TAG)),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[test]
    fn test_inject_tag_occupied() -> Result<()> {
        let mut i = Inject::default();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))?;

        // Inject the same type a second time
        let result = i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()));

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

    #[test]
    fn test_get_tag_opt_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(&SERVICE_TAG, TestService::new(expected.clone()))?;

        let result = i.get_opt(&SERVICE_TAG)?.unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_tag_opt_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_opt(&SERVICE_TAG)?;

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_get_tag_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(&SERVICE_TAG, TestService::new(expected.clone()))?;

        let result = i.get(&SERVICE_TAG)?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_dyn_tag_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject::<Arc<dyn HasId>>(&DYN_TAG, Arc::new(TestService::new(expected.clone())))?;

        let result = i.get(&DYN_TAG)?;

        assert_eq!(expected, result.get_id());

        Ok(())
    }

    #[test]
    fn test_get_tag_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get(&SERVICE_TAG);

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

    #[test]
    fn test_replace_tag_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))?;

        // Override the instance that was injected the first time
        i.replace(&SERVICE_TAG, TestService::new(expected.clone()))?;

        let result = i.get(&SERVICE_TAG)?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_replace_not_found() -> Result<()> {
        let mut i = Inject::default();

        i.inject(&SERVICE_TAG, TestService::new(fake::uuid::UUIDv4.fake()))?;

        // Override a type that doesn't have any instances yet
        let result = i.replace(&OTHER_TAG, OtherService::new(fake::uuid::UUIDv4.fake()));

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
