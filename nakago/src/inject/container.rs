use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    future::ready,
    pin::Pin,
    sync::Arc,
};

use async_trait::async_trait;
use futures::{future::Shared, Future, FutureExt};
use tokio::sync::{Mutex, MutexGuard};

use super::{Error, Key, Result};

/// The injection Container
#[derive(Default)]
pub struct Inject<'a>(pub(crate) HashMap<Key, Mutex<Injector<'a>>>);

// An Injector is a wrapper around a Dependency that can be in one of two states:
//   - Provider: The Dependency has not been requested yet, and a Provider function is available
//     to create the Dependency when it is requested.
//   - Pending: The Dependency has been requested, and is wrapped in a Shared Promise that will
//     resolve to the Dependency when it is ready.
pub(crate) struct Injector<'a> {
    value: Value<'a>,
}

#[derive(Clone)]
enum Value<'a> {
    Provider(Arc<dyn Provider<'a, Dependency>>),
    Pending(Shared<Pending<'a>>),
}

/// A Dependency that can be injected into the container
pub type Dependency = dyn Any + Send + Sync;

/// A Future that will resolve to a Dependency
pub type Pending<'a> = Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send + 'a>>;

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<'a, T: Any + Send + Sync + ?Sized>: Send + Sync {
    /// Provide a dependency for the container
    async fn provide(&'a self, i: &'a Inject<'a>) -> Result<Arc<T>>;
}

#[async_trait]
impl<'a, T: Any + Send + Sync> Provider<'a, dyn Any + Send + Sync> for T {
    /// Provide a dependency for the container
    async fn provide(&'a self, i: &'a Inject<'a>) -> Result<Arc<dyn Any + Send + Sync>> {
        self.provide(i).await
    }
}

impl<'a> Injector<'a> {
    fn from_pending(pending: Shared<Pending<'a>>) -> Self {
        Self {
            value: Value::Pending(pending),
        }
    }

    pub(crate) fn from_provider(provider: Arc<dyn Provider<'a, Dependency>>) -> Self {
        Self {
            value: Value::Provider(provider),
        }
    }

    fn request(&'a mut self, inject: &'a Inject<'a>) -> Shared<Pending<'a>> {
        self.value = Value::Pending(match &self.value {
            // If this is a Dependency that has already been requested, it will already be in a
            // Pending state. In that cose, clone the inner Shared Promise (which clones the
            // inner Arc around the Dependency at the time it's resolved).
            Value::Pending(pending) => pending.clone(),
            // If this Dependency hasn't been requested yet, kick off the inner Shared Promise,
            // which is a Provider that will resolve the Promise with the Dependency inside
            // an Arc.
            Value::Provider(provider) => provider.provide(inject).shared(),
        });

        if let Value::Pending(pending) = &self.value {
            return pending.clone();
        } else {
            unreachable!()
        }
    }
}

// The base methods powering both the Tag and TypeId modes
impl<'a> Inject<'a> {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub(crate) async fn get_key<T: Any + Send + Sync>(&'a self, key: Key) -> Result<Arc<T>> {
        let available = self.available_type_names();

        if let Some(dep) = self.get_key_opt::<T>(key.clone()).await? {
            Ok(dep)
        } else {
            Err(Error::NotFound {
                missing: key,
                available,
            })
        }
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub(crate) async fn get_key_opt<T: Any + Send + Sync>(
        &'a self,
        key: Key,
    ) -> Result<Option<Arc<T>>> {
        if let Some(injector) = self.0.get(&key) {
            let mut value: MutexGuard<'a, Injector<'a>> = injector.lock().await;
            let temp = value.request(self);

            return temp
                .await?
                .downcast::<T>()
                .map(Some)
                .map_err(|_err| Error::TypeMismatch {
                    key,
                    type_name: std::any::type_name::<T>().to_string(),
                });
        }

        Ok(None)
    }

    /// Provide a dependency directly
    pub(crate) fn inject_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let pending: Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>> =
                    Box::pin(ready::<Result<Arc<dyn Any + Send + Sync>>>(Ok(Arc::new(
                        dep,
                    ))));

                let _ = entry.insert(Mutex::new(Injector::from_pending(pending.shared())));

                Ok(())
            }
        }
    }

    /// Replace an existing dependency directly
    pub(crate) fn replace_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let pending: Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>> =
                    Box::pin(ready::<Result<Arc<dyn Any + Send + Sync>>>(Ok(Arc::new(
                        dep,
                    ))));

                let _ = entry.insert(Mutex::new(Injector::from_pending(pending.shared())));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            }),
        }
    }

    pub(crate) fn provide_key(
        &mut self,
        key: Key,
        provider: Arc<dyn Provider<'a, Dependency>>,
    ) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let _ = entry.insert(Mutex::new(Injector::from_provider(provider)));

                Ok(())
            }
        }
    }

    pub(crate) fn replace_key_with(
        &mut self,
        key: Key,
        provider: Arc<dyn Provider<'a, Dependency>>,
    ) -> Result<()> {
        match self.0.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let _ = entry.insert(Mutex::new(Injector::from_provider(provider)));

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
    Error::Provider(Arc::new(e.into()))
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
    impl<'a> Provider<'a, TestService> for TestServiceProvider {
        async fn provide(&'a self, _i: &'a Inject<'a>) -> Result<Arc<TestService>> {
            Ok(Arc::new(TestService::new(self.id.clone())))
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
    impl<'a> Provider<'a, OtherService> for OtherServiceProvider {
        async fn provide(&'a self, _i: &'a Inject<'a>) -> Result<Arc<OtherService>> {
            Ok(Arc::new(OtherService::new(self.id.clone())))
        }
    }

    #[async_trait]
    impl<'a> Provider<'a, dyn HasId> for OtherServiceProvider {
        async fn provide(&'a self, _i: &'a Inject<'a>) -> Result<Arc<dyn HasId>> {
            Ok(Arc::new(OtherService::new(self.id.clone())))
        }
    }

    #[derive(Default)]
    pub struct TestServiceHasIdProvider {}

    #[async_trait]
    impl<'a> Provider<'a, dyn HasId> for TestServiceHasIdProvider {
        async fn provide(&'a self, i: &'a Inject<'a>) -> Result<Arc<dyn HasId>> {
            // Trigger a borrow so that the reference to `Inject` has to be held across the await
            // point below, to test issues with Inject thread safety.
            let _ = i.get_type::<String>().await;

            sleep(Duration::from_millis(1)).await;

            Ok(Arc::new(OtherService::new("test-service".to_string())))
        }
    }

    // TODO: Re-implement these tests

    #[tokio::test]
    async fn test_provide_success() -> Result<()> {
        let mut i = Inject::default();

        i.provide_type::<TestService>(Arc::new(TestServiceProvider::new(
            fake::uuid::UUIDv4.fake(),
        )))?;

        assert!(
            i.0.contains_key(&Key::from_type_id::<Arc<TestService>>()),
            "key does not exist in injection container"
        );

        let _ = i.get_type::<Box<TestService>>().await?;

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
