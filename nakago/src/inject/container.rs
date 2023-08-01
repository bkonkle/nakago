use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
};

use async_trait::async_trait;

use super::{Error, Key, Result};

/// A type map for dependency injection
pub(crate) type TypeMap = HashMap<Key, Injector>;

/// The injection Container
#[derive(Default)]
pub struct Inject(pub(crate) TypeMap);

pub(crate) struct Injector {
    pub(crate) provider: Option<Box<dyn Provider<Box<dyn Any + Send + Sync>>>>,
    pub(crate) value: Option<Box<dyn Any + Send + Sync>>,
}

impl Injector {
    fn new(provider: Box<dyn Provider<Box<dyn Any + Send + Sync>>>) -> Self {
        Self {
            provider: Some(provider),
            value: None,
        }
    }

    fn from_value<T: Any + Send + Sync>(value: T) -> Self {
        Self {
            provider: None,
            value: Some(Box::new(value)),
        }
    }
}

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<T>: Any + Send + Sync
where
    T: Any + Send + Sync,
{
    /// Provide a dependency for the container
    async fn provide(&self, i: &Inject) -> Result<T>;
}

// The base methods powering both the Tag and TypeId modes
impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub(crate) fn get_key<T: Any + Send + Sync>(&self, key: Key) -> Result<&T> {
        if let Some(d) = self.get_key_opt::<T>(key.clone())? {
            Ok(d)
        } else {
            Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            })
        }
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub(crate) fn get_key_opt<T: Any + Send + Sync>(&self, key: Key) -> Result<Option<&T>> {
        if let Some(injector) = self.0.get(&key) {
            if let Some(value) = &injector.value {
                if let Some(dep) = value.downcast_ref::<T>() {
                    return Ok(Some(dep));
                } else {
                    return Err(Error::TypeMismatch {
                        key,
                        type_name: std::any::type_name::<T>().to_string(),
                    });
                }
            }
        }

        Ok(None)
    }

    /// Provide a dependency directly
    pub(crate) fn inject_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let _ = entry.insert(Injector::from_value(dep));

                Ok(())
            }
        }
    }

    /// Replace an existing dependency directly
    pub(crate) fn replace_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let _ = entry.insert(Injector::from_value(dep));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            }),
        }
    }

    pub(crate) async fn invoke_key<T: Any + Send + Sync>(&mut self, key: Key) -> Result<&T> {
        let available = self.available_type_names();

        if let Entry::Occupied(mut entry) = self.0.entry(key.clone()) {
            if let Some(value) = &entry.get().value {
                if let Some(dep) = value.downcast_ref::<T>() {
                    return Ok(dep);
                } else {
                    return Err(Error::TypeMismatch {
                        key,
                        type_name: std::any::type_name::<T>().to_string(),
                    });
                }
            } else if let Some(provider) = &entry.get().provider {
                if let Ok(dep) = provider.provide(self).await?.downcast::<T>() {
                    entry.insert(Injector::from_value(*dep));
                } else {
                    return Err(Error::TypeMismatch {
                        key,
                        type_name: std::any::type_name::<T>().to_string(),
                    });
                };
            }
        }

        return Err(Error::NotFound {
            missing: key,
            available,
        });
    }

    pub(crate) fn provide_key(
        &mut self,
        key: Key,
        provider: Box<dyn Provider<Box<dyn Any + Send + Sync>>>,
    ) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let _ = entry.insert(Injector::new(provider));

                Ok(())
            }
        }
    }

    pub(crate) fn replace_key_provider(
        &mut self,
        key: Key,
        provider: Box<dyn Provider<Box<dyn Any + Send + Sync>>>,
    ) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let _ = entry.insert(Injector::new(provider));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            }),
        }
    }

    /// Return a list of all available type names in the map
    pub(crate) fn available_type_names(&self) -> Vec<Key> {
        self.0.keys().cloned().collect()
    }
}

/// Wrap an error that can be converted into an Anyhow error with an inject Provider error
pub fn to_provider_error<E>(e: E) -> Error
where
    anyhow::Error: From<E>,
{
    Error::Provider(e.into())
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    // TODO: Uncomment when tests below are re-implemented
    // use crate::inject::{
    //     tag::test::{DYN_TAG, OTHER_TAG, SERVICE_TAG},
    //     Key,
    // };

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
    impl Provider<Box<dyn Any + Send + Sync>> for TestServiceProvider {
        async fn provide(&self, _i: &Inject) -> Result<Box<dyn Any + Send + Sync>> {
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
            let _ = i.get_type::<String>();

            sleep(Duration::from_millis(1)).await;

            Ok(Arc::new(OtherService::new("test-service".to_string())))
        }
    }

    // TODO: Re-implement these tests

    #[tokio::test]
    async fn test_provide_success() -> Result<()> {
        let mut i = Inject::default();

        i.provide_type::<Box<TestService>>(Box::new(TestServiceProvider::new(
            fake::uuid::UUIDv4.fake(),
        )))?;

        assert!(
            i.0.contains_key(&Key::from_type_id::<Box<TestService>>()),
            "key does not exist in injection container"
        );

        let _ = i.get_type::<Box<TestService>>()?;

        Ok(())
    }

    // #[tokio::test]
    // async fn test_provide_dyn_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     i.provide_type_old(TestServiceHasIdProvider::default())
    //         .await?;

    //     assert!(
    //         i.0.contains_key(&Key::from_type_id::<Arc<dyn HasId>>()),
    //         "key does not exist in injection container"
    //     );

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_provide_occupied() -> Result<()> {
    //     let mut i = Inject::default();

    //     i.provide_type_old(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
    //         .await?;

    //     let result = i
    //         .provide_type_old(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
    //         .await;

    //     if let Err(err) = result {
    //         assert_eq!(
    //             format!("{} has already been provided", type_name::<TestService>()),
    //             err.to_string()
    //         );
    //     } else {
    //         panic!("did not return Err as expected")
    //     }

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_replace_with_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     let expected: String = fake::uuid::UUIDv4.fake();

    //     i.provide_type_old(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
    //         .await?;

    //     // Override the instance that was injected the first time
    //     i.replace_type_with_old(TestServiceProvider::new(expected.clone()))
    //         .await?;

    //     let result = i.get_type::<TestService>()?;

    //     assert_eq!(expected, result.id);

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_replace_with_not_found() -> Result<()> {
    //     let mut i = Inject::default();

    //     i.provide_type_old(TestServiceProvider::new(fake::uuid::UUIDv4.fake()))
    //         .await?;

    //     // Override a type that doesn't have any instances yet
    //     let result = i
    //         .replace_type_with_old::<OtherService, _>(OtherServiceProvider::new(
    //             fake::uuid::UUIDv4.fake(),
    //         ))
    //         .await;

    //     if let Err(err) = result {
    //         assert_eq!(
    //             format!(
    //                 "{} was not found\n\nAvailable:\n - {}",
    //                 type_name::<OtherService>(),
    //                 type_name::<TestService>()
    //             ),
    //             err.to_string()
    //         );
    //     } else {
    //         panic!("did not return Err as expected")
    //     }

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_provide_tag_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     i.provide_old(
    //         &SERVICE_TAG,
    //         TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
    //     )
    //     .await?;

    //     assert!(
    //         i.0.contains_key(&Key::from_tag::<TestService>(&SERVICE_TAG)),
    //         "key does not exist in injection container"
    //     );

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_provide_tag_dyn_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     i.provide_old(&DYN_TAG, TestServiceHasIdProvider::default())
    //         .await?;

    //     assert!(
    //         i.0.contains_key(&Key::from_tag::<Arc<dyn HasId>>(&DYN_TAG)),
    //         "key does not exist in injection container"
    //     );

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_provide_tag_occupied() -> Result<()> {
    //     let mut i = Inject::default();

    //     i.provide_old(
    //         &SERVICE_TAG,
    //         TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
    //     )
    //     .await?;

    //     let result = i
    //         .provide_old(
    //             &SERVICE_TAG,
    //             TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
    //         )
    //         .await;

    //     if let Err(err) = result {
    //         assert_eq!(
    //             format!("{} has already been provided", SERVICE_TAG),
    //             err.to_string()
    //         );
    //     } else {
    //         panic!("did not return Err as expected")
    //     }

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_replace_tag_with_success() -> Result<()> {
    //     let mut i = Inject::default();

    //     let expected: String = fake::uuid::UUIDv4.fake();

    //     i.provide_old(
    //         &SERVICE_TAG,
    //         TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
    //     )
    //     .await?;

    //     // Override the instance that was injected the first time
    //     i.replace_with_old(&SERVICE_TAG, TestServiceProvider::new(expected.clone()))
    //         .await?;

    //     let result = i.get(&SERVICE_TAG)?;

    //     assert_eq!(expected, result.id);

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_replace_tag_with_not_found() -> Result<()> {
    //     let mut i = Inject::default();

    //     i.provide_old(
    //         &SERVICE_TAG,
    //         TestServiceProvider::new(fake::uuid::UUIDv4.fake()),
    //     )
    //     .await?;

    //     // Override a type that doesn't have any instances yet
    //     let result = i
    //         .replace_with_old(
    //             &OTHER_TAG,
    //             OtherServiceProvider::new(fake::uuid::UUIDv4.fake()),
    //         )
    //         .await;

    //     if let Err(err) = result {
    //         assert_eq!(
    //             format!(
    //                 "{} was not found\n\nAvailable:\n - {}",
    //                 OTHER_TAG, SERVICE_TAG
    //             ),
    //             err.to_string()
    //         );
    //     } else {
    //         panic!("did not return Err as expected")
    //     }

    //     Ok(())
    // }
}
