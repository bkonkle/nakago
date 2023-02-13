use fnv::FnvHashMap;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::{any::Any, fmt::Debug, ops::Deref};

use super::{Error, Key, Result};

/// A type map for dependency injection
pub(crate) type TypeMap = FnvHashMap<Key, Box<dyn Any>>;

/// The injection Container
#[derive(Default, Debug)]
pub struct Inject(pub(crate) RwLock<TypeMap>);

impl Deref for Inject {
    type Target = RwLock<TypeMap>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    // pub fn get<T: Send + Sync + 'static>(&self) -> Result<&T> {
    //     self.get_opt::<T>().ok_or_else(|| Error::NotFound {
    //         missing: Key::from_type_id::<T>(),
    //         available: self.available_type_names(),
    //     })
    // }

    /// Retrieve a mutable reference to a dependency if it exists, and return an error otherwise
    // pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Result<&mut T> {
    //     // TODO: Since `self` is borrowed as a mutable ref for `self.get_mut_opt()`, it cannot be
    //     // used for self.available_type_names() within the `.ok_or_else()` call below. Because of
    //     // this, the `available` property is pre-loaded here in case there is an error. It must
    //     // iterate over the keys of the map to do this - which is minor, but I'd still like to
    //     // avoid it.
    //     let available = self.available_type_names();

    //     (*self.get_mut_opt::<T>()).ok_or_else(|| Error::NotFound {
    //         missing: Key::from_type_id::<T>(),
    //         available,
    //     })
    // }

    /// Retrieve a reference to a dependency if it exists in the map
    pub fn get<T: Any>(&self) -> Result<MappedRwLockReadGuard<'_, &T>> {
        let key = Key::from_type_id::<T>();

        RwLockReadGuard::try_map(self.0.read(), |m| {
            m.get(&key).and_then(|b| b.downcast_ref())
        })
        .map_err(|_err| Error::NotFound {
            missing: Key::from_type_id::<T>(),
            available: self.available_type_names(),
        })
    }

    // /// Retrieve a mutable reference to a dependency if it exists in the map
    // pub fn get_mut<T: Any>(&self) -> Result<MappedRwLockWriteGuard<'_, &mut T>> {
    //     let key = Key::from_type_id::<T>();

    //     RwLockWriteGuard::try_map(self.0.write(), |m| {
    //         m.get_mut(&key).and_then(|b| b.downcast_mut())
    //     })
    //     .map_err(|_| Error::NotFound {
    //         missing: Key::from_type_id::<T>(),
    //         available: self.available_type_names(),
    //     })
    // }

    /// Provide a dependency directly
    pub fn inject<T: Any>(&mut self, dep: T) -> Result<()> {
        let key = Key::from_type_id::<T>();

        if self.0.read().contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let _ = self.0.write().insert(key, Box::new(dep));

        Ok(())
    }

    /// Replace an existing dependency directly
    pub fn replace<T: Any>(&mut self, dep: T) -> Result<()> {
        let key = Key::from_type_id::<T>();

        if !self.0.read().contains_key(&key) {
            return Err(Error::NotFound {
                missing: Key::from_type_id::<T>(),
                available: self.available_type_names(),
            });
        }

        self.0.write().insert(key, Box::new(dep));

        Ok(())
    }

    pub(crate) fn available_type_names(&self) -> Vec<Key> {
        self.0.read().keys().cloned().collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::any::type_name;

    use super::*;

    pub trait HasId: Sync + Send {
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

    // #[test]
    // fn test_get_opt_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     let expected: String = fake::uuid::UUIDv4.fake();

    //     i.inject(Box::new(TestService::new(expected.clone())))?;

    //     let result = i.get_opt::<TestService>().unwrap();

    //     assert_eq!(expected, result.id);

    //     Ok(())
    // }

    // #[test]
    // fn test_get_opt_vec_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     let expected: String = fake::uuid::UUIDv4.fake();

    //     i.inject(Box::new(vec![TestService::new(expected.clone())]))?;

    //     let result = i.get_opt::<Vec<TestService>>().unwrap();

    //     assert_eq!(expected, result[0].id);

    //     Ok(())
    // }

    // #[test]
    // fn test_get_opt_not_found() -> Result<()> {
    //     let i = Inject::default();

    //     let result = i.get_opt::<TestService>();

    //     assert!(result.is_none());

    //     Ok(())
    // }

    #[test]
    fn test_get_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(Box::new(TestService::new(expected.clone())))?;

        let result = i.get::<TestService>()?;

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_dyn_get_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject::<Box<dyn HasId>>(Box::new(TestService::new(expected.clone())))?;

        let repo = i.get::<Box<dyn HasId>>()?;

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

    // #[test]
    // fn test_get_mut_opt_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     let expected: String = fake::uuid::UUIDv4.fake();

    //     i.inject(Box::new(TestService::new(expected.clone())))?;

    //     let result = i.get_mut_opt::<TestService>().unwrap();

    //     assert_eq!(expected, result.id);

    //     Ok(())
    // }

    // #[test]
    // fn test_get_mut_opt_not_found() -> Result<()> {
    //     let mut i = Inject::default();

    //     let result = i.get_mut_opt::<TestService>();

    //     assert!(result.is_none());

    //     Ok(())
    // }

    // #[test]
    // fn test_get_mut_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     let expected: String = fake::uuid::UUIDv4.fake();

    //     i.inject(vec![TestService::new(fake::uuid::UUIDv4.fake())])?;

    //     let mut services = i.get_mut::<Vec<TestService>>()?;
    //     services.push(TestService::new(expected.clone()));

    //     let result = i.get::<Vec<TestService>>()?;

    //     assert_eq!(expected, result[1].id);

    //     Ok(())
    // }

    // #[test]
    // fn test_get_mut_not_found() -> Result<()> {
    //     let i = Inject::default();

    //     let result = i.get_mut::<TestService>();

    //     if let Err(err) = result {
    //         assert_eq!(
    //             format!(
    //                 "{} was not found\n\nAvailable: (empty)",
    //                 type_name::<TestService>(),
    //             ),
    //             err.to_string()
    //         );
    //     } else {
    //         panic!("did not return Err as expected")
    //     }

    //     Ok(())
    // }

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
                    type_name::<OtherService>(),
                    type_name::<TestService>(),
                    type_name::<dyn HasId>()
                ),
                err.to_string()
            );
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }
}
