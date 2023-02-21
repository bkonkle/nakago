use fnv::FnvHashMap;
use std::{any::Any, fmt::Debug};

use super::{Error, Key, Result};

/// A type map for dependency injection
pub(crate) type TypeMap = FnvHashMap<Key, Box<dyn Any + Send + Sync>>;

/// The injection Container
#[derive(Default, Debug)]
pub struct Inject(pub(crate) TypeMap);

impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub fn get<T: Any + Send + Sync>(&self) -> Result<&T> {
        self.get_key(Key::from_type_id::<T>())
    }

    /// Retrieve a mutable reference to a dependency if it exists, and return an error otherwise
    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Result<&mut T> {
        self.get_key_mut(Key::from_type_id::<T>())
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub fn get_opt<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.get_key_opt(Key::from_type_id::<T>())
    }

    /// Retrieve a mutable reference to a dependency if it exists in the map
    pub fn get_mut_opt<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.get_key_mut_opt(Key::from_type_id::<T>())
    }

    /// Provide a dependency directly
    pub fn inject<T: Any + Send + Sync>(&mut self, dep: T) -> Result<()> {
        self.inject_key(Key::from_type_id::<T>(), dep)
    }

    /// Replace an existing dependency directly
    pub fn replace<T: Any + Send + Sync>(&mut self, dep: T) -> Result<()> {
        self.replace_key(Key::from_type_id::<T>(), dep)
    }

    /// Consume a dependency, removing it from the container and moving it to the caller
    pub fn consume<T: Any + Send + Sync>(&mut self) -> Result<T> {
        self.consume_key(Key::from_type_id::<T>())
    }

    // The base methods powering both the Tag and TypeId modes

    pub(crate) fn get_key<T: Any + Send + Sync>(&self, key: Key) -> Result<&T> {
        self.get_key_opt::<T>(key.clone())
            .ok_or_else(|| Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            })
    }

    /// Retrieve a mutable reference to a dependency if it exists, and return an error otherwise
    pub(crate) fn get_key_mut<T: Any + Send + Sync>(&mut self, key: Key) -> Result<&mut T> {
        // TODO: Since `self` is borrowed as a mutable ref for `self.get_mut_opt()`, it cannot be
        // used for self.available_type_names() within the `.ok_or_else()` call below. Because of
        // this, the `available` property is pre-loaded here in case there is an error. It must
        // iterate over the keys of the map to do this - which is minor, but I'd still like to
        // avoid it.
        let available = self.available_type_names();

        self.get_key_mut_opt::<T>(key.clone())
            .ok_or(Error::NotFound {
                missing: key,
                available,
            })
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub(crate) fn get_key_opt<T: Any + Send + Sync>(&self, key: Key) -> Option<&T> {
        self.0.get(&key).and_then(|d| d.downcast_ref::<T>())
    }

    /// Retrieve a mutable reference to a dependency if it exists in the map
    pub(crate) fn get_key_mut_opt<T: Any + Send + Sync>(&mut self, key: Key) -> Option<&mut T> {
        self.0.get_mut(&key).and_then(|d| d.downcast_mut::<T>())
    }

    /// Provide a dependency directly
    pub(crate) fn inject_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if self.0.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let _ = self.0.insert(key, Box::new(dep));

        Ok(())
    }

    /// Replace an existing dependency directly
    pub(crate) fn replace_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if !self.0.contains_key(&key) {
            return Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            });
        }

        self.0.insert(key, Box::new(dep));

        Ok(())
    }

    pub(crate) fn consume_key<T: Any + Send + Sync>(&mut self, key: Key) -> Result<T> {
        self.0
            .remove(&key)
            .ok_or_else(|| Error::NotFound {
                missing: key.clone(),
                available: self.available_type_names(),
            })
            .and_then(|d| d.downcast().map_err(|_err| Error::CannotConsume(key)))
            .map(|d| *d)
    }

    pub(crate) fn available_type_names(&self) -> Vec<Key> {
        self.0.keys().cloned().collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::any::type_name;

    use super::*;

    pub trait HasId: Send + Sync {
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
            i.0.contains_key(&Key::from_type_id::<TestService>()),
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
    fn test_get_opt_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(TestService::new(expected.clone()))?;

        let result = i.get_opt::<TestService>().unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_opt_vec_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(vec![TestService::new(expected.clone())])?;

        let result = i.get_opt::<Vec<TestService>>().unwrap();

        assert_eq!(expected, result[0].id);

        Ok(())
    }

    #[test]
    fn test_get_opt_not_found() -> Result<()> {
        let i = Inject::default();

        let result = i.get_opt::<TestService>();

        assert!(result.is_none());

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

    #[test]
    fn test_get_mut_opt_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(TestService::new(expected.clone()))?;

        let result = i.get_mut_opt::<TestService>().unwrap();

        assert_eq!(expected, result.id);

        Ok(())
    }

    #[test]
    fn test_get_mut_opt_not_found() -> Result<()> {
        let mut i = Inject::default();

        let result = i.get_mut_opt::<TestService>();

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_get_mut_success() -> Result<()> {
        let mut i = Inject::default();

        let expected: String = fake::uuid::UUIDv4.fake();

        i.inject(vec![TestService::new(fake::uuid::UUIDv4.fake())])?;

        let services = i.get_mut::<Vec<TestService>>()?;
        services.push(TestService::new(expected.clone()));

        let result = i.get::<Vec<TestService>>()?;

        assert_eq!(expected, result[1].id);

        Ok(())
    }

    #[test]
    fn test_get_mut_not_found() -> Result<()> {
        let mut i = Inject::default();

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
        i.replace(TestService {
            id: expected.clone(),
        })?;

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
        let result = i.replace(Box::new(OtherService {
            other_id: fake::uuid::UUIDv4.fake(),
        }));

        if let Err(err) = result {
            let message = err.to_string();

            assert!(message.contains(&format!(
                "{} was not found\n\nAvailable:",
                type_name::<Box<OtherService>>()
            )));

            assert!(message.contains(&format!("\n - {}", type_name::<Box<TestService>>())));
            assert!(message.contains(&format!("\n - {}", type_name::<Box<dyn HasId>>())));
        } else {
            panic!("did not return Err as expected")
        }

        Ok(())
    }
}
