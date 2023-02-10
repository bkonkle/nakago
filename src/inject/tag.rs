use std::{any::Any, fmt::Display, marker::PhantomData, ops::Deref};

use super::{Error, Key};
use crate::Inject;

/// A dependency injection Tag representing a specific type
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
    pub fn get_tag<T: Any>(&self, tag: &'static Tag<T>) -> Result<&T, Error> {
        self.get_tag_opt::<T>(tag).ok_or_else(|| Error::NotFound {
            missing: Key::from_tag::<T>(tag),
            available: self.available_type_names(),
        })
    }

    /// Retrieve a mutable reference to a dependency if it exists, and return an error otherwise
    pub fn get_tag_mut<T: Any>(&mut self, tag: &'static Tag<T>) -> Result<&mut T, Error> {
        // TODO: Move this into .ok_or_else()
        let available = self.available_type_names();

        self.get_tag_mut_opt::<T>(tag)
            .ok_or_else(|| Error::NotFound {
                missing: Key::from_tag::<T>(tag),
                available,
            })
    }

    /// Retrieve a reference to a tagged dependency if it exists in the map
    pub fn get_tag_opt<T: Any>(&self, tag: &'static Tag<T>) -> Option<&T> {
        let key = Key::from_tag::<T>(tag);

        self.0.get(&key).and_then(|d| d.downcast_ref::<T>())
    }

    /// Retrieve a mutable reference to a tagged dependency if it exists in the map
    pub fn get_tag_mut_opt<T: Any>(&mut self, tag: &'static Tag<T>) -> Option<&mut T> {
        let key = Key::from_tag::<T>(tag);

        self.0.get_mut(&key).and_then(|d| d.downcast_mut::<T>())
    }

    /// Provide a tagged dependency directly
    pub fn inject_tag<T: Any + Sync + Send>(
        &mut self,
        dep: T,
        tag: &'static Tag<T>,
    ) -> Result<(), Error> {
        let key = Key::from_tag::<T>(tag);

        if self.0.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let _ = self.0.insert(key, Box::new(dep));

        Ok(())
    }

    /// Replace an existing tagged dependency directly
    pub fn replace_tag<T: Any + Sync + Send>(
        &mut self,
        dep: T,
        tag: &'static Tag<T>,
    ) -> Result<(), Error> {
        let key = Key::from_tag::<T>(tag);

        if !self.0.contains_key(&key) {
            return Err(Error::NotFound {
                missing: Key::from_tag::<T>(tag),
                available: self.available_type_names(),
            });
        }

        self.0.insert(key, Box::new(dep));

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;

    use super::*;
    use crate::inject::container::test::{OtherService, TestService};

    pub const SERVICE_TAG: Tag<TestService> = Tag::new("InMemoryTestService");
    pub const OTHER_TAG: Tag<OtherService> = Tag::new("InMemoryOtherService");

    #[test]
    fn test_inject_tag_success() -> anyhow::Result<()> {
        let mut i = Inject::default();

        i.inject_tag(TestService::new(fake::uuid::UUIDv4.fake()), &SERVICE_TAG)?;

        assert!(
            i.0.contains_key(&Key::from_tag::<TestService>(&SERVICE_TAG)),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[test]
    fn test_inject_tag_occupied() -> anyhow::Result<()> {
        let mut i = Inject::default();

        i.inject_tag(TestService::new(fake::uuid::UUIDv4.fake()), &SERVICE_TAG)?;

        // Inject the same type a second time
        let result = i.inject_tag(TestService::new(fake::uuid::UUIDv4.fake()), &SERVICE_TAG);

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
    fn test_get_tag_opt_success() -> anyhow::Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_tag(TestService::new(expected.clone()), &SERVICE_TAG)?;

        let result = i.get_tag_opt::<TestService>(&SERVICE_TAG).unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_tag_opt_not_found() -> anyhow::Result<()> {
        let i = Inject::default();

        let result = i.get_tag_opt::<TestService>(&SERVICE_TAG);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_get_tag_success() -> anyhow::Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_tag(TestService::new(expected.clone()), &SERVICE_TAG)?;

        let result = i.get_tag::<TestService>(&SERVICE_TAG)?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_tag_not_found() -> anyhow::Result<()> {
        let i = Inject::default();

        let result = i.get_tag::<TestService>(&SERVICE_TAG);

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
    fn test_get_tag_mut_opt_success() -> anyhow::Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_tag(TestService::new(expected.clone()), &SERVICE_TAG)?;

        let result = i.get_tag_mut_opt::<TestService>(&SERVICE_TAG).unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_tag_mut_opt_not_found() -> anyhow::Result<()> {
        let mut i = Inject::default();

        let result = i.get_tag_mut_opt::<TestService>(&SERVICE_TAG);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_get_tag_mut_success() -> anyhow::Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_tag(TestService::new(expected.clone()), &SERVICE_TAG)?;

        let result = i.get_tag_mut::<TestService>(&SERVICE_TAG)?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_tag_mut_not_found() -> anyhow::Result<()> {
        let mut i = Inject::default();

        let result = i.get_tag_mut::<TestService>(&SERVICE_TAG);

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
    fn test_replace_tag_success() -> anyhow::Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject_tag(TestService::new(fake::uuid::UUIDv4.fake()), &SERVICE_TAG)?;

        // Override the instance that was injected the first time
        i.replace_tag(TestService::new(expected.clone()), &SERVICE_TAG)?;

        let result = i.get_tag::<TestService>(&SERVICE_TAG)?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_replace_not_found() -> anyhow::Result<()> {
        let mut i = Inject::default();

        i.inject_tag(TestService::new(fake::uuid::UUIDv4.fake()), &SERVICE_TAG)?;

        // Override a type that doesn't have any instances yet
        let result = i.replace_tag(OtherService::new(fake::uuid::UUIDv4.fake()), &OTHER_TAG);

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
