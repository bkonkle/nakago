use fnv::FnvHashMap;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::{any::Any, fmt::Debug};

use super::{Error, Key, Result};

/// A type map for dependency injection
pub(crate) type TypeMap = FnvHashMap<Key, Box<dyn Any>>;

/// The injection Container
#[derive(Default, Debug)]
pub struct Inject(pub(crate) RwLock<TypeMap>);

impl Inject {
    /// Retrieve a reference to a dependency if it exists in the map
    pub fn get<T: Any>(&self) -> Result<MappedRwLockReadGuard<'_, T>> {
        self.get_key(Key::from_type_id::<T>())
    }

    /// Retrieve a mutable reference to a dependency if it exists in the map
    pub fn get_mut<T: Any>(&self) -> Result<MappedRwLockWriteGuard<'_, T>> {
        self.get_key_mut(Key::from_type_id::<T>())
    }

    /// Provide a dependency directly
    pub fn inject<T: Any>(&mut self, dep: T) -> Result<()> {
        self.inject_key(Key::from_type_id::<T>(), dep)
    }

    /// Replace an existing dependency directly
    pub fn replace<T: Any>(&mut self, dep: T) -> Result<()> {
        self.replace_key(Key::from_type_id::<T>(), dep)
    }

    // The base methods powering both the Tag and TypeId modes

    pub(crate) fn get_key<T: Any>(&self, key: Key) -> Result<MappedRwLockReadGuard<'_, T>> {
        RwLockReadGuard::try_map(self.0.read(), |m| {
            m.get(&key).and_then(|b| b.downcast_ref())
        })
        .map_err(|m| Error::NotFound {
            missing: key,
            available: m.keys().cloned().collect(),
        })
    }

    pub(crate) fn get_key_mut<T: Any>(&self, key: Key) -> Result<MappedRwLockWriteGuard<'_, T>> {
        RwLockWriteGuard::try_map(self.0.write(), |m| {
            m.get_mut(&key).and_then(|b| b.downcast_mut())
        })
        .map_err(|m| Error::NotFound {
            missing: key,
            available: m.keys().cloned().collect(),
        })
    }

    pub(crate) fn inject_key<T: Any>(&mut self, key: Key, dep: T) -> Result<()> {
        if self.0.read().contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let _ = self.0.write().insert(key, Box::new(dep));

        Ok(())
    }

    pub(crate) fn replace_key<T: Any>(&mut self, key: Key, dep: T) -> Result<()> {
        if !self.0.read().contains_key(&key) {
            return Err(Error::NotFound {
                missing: key,
                available: self.0.read().keys().cloned().collect(),
            });
        }

        self.0.write().insert(key, Box::new(dep));

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::{any::type_name, sync::Arc};

    use super::*;

    pub trait HasId: Send {
        fn get_id(&self) -> String;
    }

    pub struct TestService {
        pub(crate) id: String,
    }

    impl TestService {
        pub fn new(id: String) -> Self {
            Self { id }
        }
    }

    impl HasId for TestService {
        fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    pub struct OtherService {
        pub(crate) other_id: String,
    }

    impl OtherService {
        pub fn new(other_id: String) -> Self {
            Self { other_id }
        }
    }

    impl HasId for OtherService {
        fn get_id(&self) -> String {
            self.other_id.clone()
        }
    }

    #[test]
    fn test_inject_success() -> Result<()> {
        let mut i = Inject::default();

        let service = TestService::new(fake::uuid::UUIDv4.fake());

        i.inject(service)?;

        assert!(
            i.0.read().contains_key(&Key::from_type_id::<TestService>()),
            "key does not exist in injection container"
        );

        Ok(())
    }

    #[test]
    fn test_inject_occupied() -> Result<()> {
        let mut i = Inject::default();

        i.inject(TestService::new(fake::uuid::UUIDv4.fake()))?;

        // Inject the same type a second time
        let result = i.inject(TestService::new(fake::uuid::UUIDv4.fake()));

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

    #[test]
    fn test_get_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(TestService::new(expected.clone()))?;

        let result = i.get::<TestService>()?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_dyn_get_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject::<Arc<dyn HasId>>(Arc::new(TestService::new(expected.clone())))?;

        let repo = i.get::<Arc<dyn HasId>>()?;

        assert_eq!(expected, repo.get_id());

        Ok(())
    }

    #[test]
    fn test_get_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get::<TestService>();

        if let Err(err) = result {
            assert_eq!(
                format!(
                    "{} was not found\n\nAvailable: (empty)",
                    type_name::<TestService>(),
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[test]
    fn test_get_mut_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(vec![TestService::new(fake::uuid::UUIDv4.fake())])?;

        let mut services = i.get_mut::<Vec<TestService>>()?;
        services.push(TestService::new(expected.clone()));

        drop(services);

        let result = i.get::<Vec<TestService>>()?;

        assert_eq!(expected, result[1].id);

        Ok(())
    }

    #[test]
    fn test_get_mut_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_mut::<TestService>();

        if let Err(err) = result {
            assert_eq!(
                format!(
                    "{} was not found\n\nAvailable: (empty)",
                    type_name::<TestService>(),
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }

    #[test]
    fn test_replace_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(TestService::new(fake::uuid::UUIDv4.fake()))?;

        // Override the instance that was injected the first time
        i.replace(TestService::new(expected.clone()))?;

        let result = i.get::<TestService>()?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_replace_not_found() -> Result<()> {
        let mut i = Inject::default();

        i.inject(Box::new(TestService::new(fake::uuid::UUIDv4.fake())))?;
        i.inject::<Box<dyn HasId>>(Box::new(OtherService::new(fake::uuid::UUIDv4.fake())))?;

        // Override a type that doesn't have any instances yet
        let result = i.replace(Box::new(OtherService::new(fake::uuid::UUIDv4.fake())));

        if let Err(err) = result {
            assert_eq!(
                format!(
                    "{} was not found\n\nAvailable:\n - {}\n\n - {}",
                    type_name::<Box<OtherService>>(),
                    type_name::<Box<dyn HasId>>(),
                    type_name::<Box<TestService>>()
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }
}
