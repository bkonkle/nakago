use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    future::ready,
    pin::Pin,
    sync::Arc,
};

use async_trait::async_trait;
use futures::{future::Shared, Future, FutureExt};
use tokio::sync::RwLock;

use super::{Error, Key, Result};

/// The injection Container
#[derive(Default, Clone)]
pub struct Inject(pub(crate) Arc<RwLock<HashMap<Key, Injector>>>);

pub(crate) struct Injector {
    value: RwLock<Value>,
}

#[derive(Clone)]
enum Value {
    Provider(Arc<dyn AnyProvider>),
    Pending(Shared<Pending>),
}

/// A Dependency that can be injected into the container
pub type Dependency = dyn Any + Send + Sync;

/// A Future that will resolve to a Dependency
pub type Pending = Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>>;

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<T: Any + Send + Sync + ?Sized>: Send + Sync {
    /// Provide a dependency for the container
    async fn provide(self, i: Inject) -> Result<Arc<T>>;
}

#[async_trait]
pub trait AnyProvider: Send + Sync {
    async fn provide_any(self, i: Inject) -> Result<Arc<Dependency>>;
}

#[async_trait]
impl<T: Any + Send + Sync> AnyProvider for Arc<dyn Provider<T>> {
    /// Provide a dependency for the container
    async fn provide_any(self, i: Inject) -> Result<Arc<Dependency>> {
        println!(">------ provide any ------<");
        let dep = self.provide(i).await?;

        Ok(dep)
    }
}

impl Injector {
    fn from_pending(pending: Shared<Pending>) -> Self {
        Self {
            value: RwLock::new(Value::Pending(pending)),
        }
    }

    pub(crate) fn from_provider(provider: Arc<dyn AnyProvider>) -> Self {
        Self {
            value: RwLock::new(Value::Provider(provider)),
        }
    }

    async fn request(&self, inject: Inject) -> Shared<Pending> {
        println!(">------ getting value ------<");

        let value = self.value.read().await;

        println!(">------ value lock acquired ------<");

        if let Value::Pending(pending) = &*value {
            println!(">------ returning already pending value ------<");
            return pending.clone();
        }

        drop(value);

        println!(">------ getting mut value ------<");

        let mut value = self.value.write().await;

        println!(">------ mut value lock acquired ------<");

        *value = Value::Pending(match value.clone() {
            Value::Pending(pending) => pending,
            Value::Provider(provider) => provider.provide_any(inject).shared(),
        });

        if let Value::Pending(pending) = &*value {
            println!(">------ returning pending value ------<");
            pending.clone()
        } else {
            // We still hold the lock and the above operation is guaranteed to return a pending
            // value, so this should not be reachable.
            unreachable!()
        }
    }
}

// The base methods powering both the Tag and TypeId modes
impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub(crate) async fn get_key<T: Any + Send + Sync>(&self, key: Key) -> Result<Arc<T>> {
        if let Some(dep) = self.get_key_opt::<T>(key.clone()).await? {
            Ok(dep)
        } else {
            println!(">------ accessing container ------<");

            let container = self.0.read().await;

            println!(">------ lock acquired ------<");

            Err(Error::NotFound {
                missing: key,
                available: container.keys().cloned().collect(),
            })
        }
    }

    /// Retrieve a reference to a dependency if it exists in the map
    pub(crate) async fn get_key_opt<T: Any + Send + Sync>(
        &self,
        key: Key,
    ) -> Result<Option<Arc<T>>> {
        println!(">------ accessing container (opt) ------<");

        let container = self.0.read().await;

        println!(">------ lock acquired (opt) ------<");

        if let Some(injector) = container.get(&key) {
            let pending = injector.request(self.clone()).await;
            println!(">- pending -> {:?}", pending);
            let value = pending.await?;
            println!(">- value -> {:?}", value);

            return value
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
    pub(crate) async fn inject_key<T: Any + Send + Sync>(&self, key: Key, dep: T) -> Result<()> {
        let mut container = self.0.write().await;

        match container.entry(key.clone()) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let pending: Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>> =
                    Box::pin(ready::<Result<Arc<dyn Any + Send + Sync>>>(Ok(Arc::new(
                        dep,
                    ))));

                let _ = entry.insert(Injector::from_pending(pending.shared()));

                Ok(())
            }
        }
    }

    /// Replace an existing dependency directly
    pub(crate) async fn replace_key<T: Any + Send + Sync>(&self, key: Key, dep: T) -> Result<()> {
        let mut container = self.0.write().await;

        match container.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let pending: Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>> =
                    Box::pin(ready::<Result<Arc<dyn Any + Send + Sync>>>(Ok(Arc::new(
                        dep,
                    ))));

                let _ = entry.insert(Injector::from_pending(pending.shared()));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available: container.keys().cloned().collect(),
            }),
        }
    }

    pub(crate) async fn provide_key(&self, key: Key, provider: Arc<dyn AnyProvider>) -> Result<()> {
        let mut container = self.0.write().await;

        match container.entry(key.clone()) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let _ = entry.insert(Injector::from_provider(provider));

                Ok(())
            }
        }
    }

    pub(crate) async fn replace_key_with(
        &self,
        key: Key,
        provider: Arc<dyn AnyProvider>,
    ) -> Result<()> {
        let mut container = self.0.write().await;

        match container.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let _ = entry.insert(Injector::from_provider(provider));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available: container.keys().cloned().collect(),
            }),
        }
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
    impl Provider<TestService> for TestServiceProvider {
        async fn provide(self, _i: Inject) -> Result<Arc<TestService>> {
            println!(">- providing -> {:?}", self.id);

            Ok(Arc::new(TestService::new(self.id)))
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
        async fn provide(self, _i: Inject) -> Result<Arc<OtherService>> {
            Ok(Arc::new(OtherService::new(self.id)))
        }
    }

    #[async_trait]
    impl Provider<dyn HasId> for OtherServiceProvider {
        async fn provide(self, _i: Inject) -> Result<Arc<dyn HasId>> {
            Ok(Arc::new(OtherService::new(self.id)))
        }
    }

    #[derive(Default)]
    pub struct TestServiceHasIdProvider {}

    #[async_trait]
    impl Provider<dyn HasId> for TestServiceHasIdProvider {
        async fn provide(self, i: Inject) -> Result<Arc<dyn HasId>> {
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
        let i = Inject::default();

        i.provide_type::<TestService>(Arc::new(TestServiceProvider::new(
            fake::uuid::UUIDv4.fake(),
        )))
        .await?;

        assert!(
            i.0.read()
                .await
                .contains_key(&Key::from_type_id::<TestService>()),
            "key does not exist in injection container"
        );

        let _ = i.get_type::<TestService>().await?;

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
