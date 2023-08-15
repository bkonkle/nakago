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
    Provider(Arc<dyn Provider>),
    Pending(Shared<Pending>),
}

/// A Dependency that can be injected into the container
pub type Dependency = dyn Any + Send + Sync;

/// A Future that will resolve to a Dependency
pub type Pending = Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>>;

/// A trait for async injection Providers
#[async_trait]
pub trait Provider: Send + Sync {
    /// Provide a dependency for the container
    async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<Dependency>>;
}

impl Injector {
    fn from_pending(pending: Shared<Pending>) -> Self {
        Self {
            value: RwLock::new(Value::Pending(pending)),
        }
    }

    pub(crate) fn from_provider(provider: impl Provider + 'static) -> Self {
        Self {
            value: RwLock::new(Value::Provider(Arc::new(provider))),
        }
    }

    async fn request(&self, inject: Inject) -> Shared<Pending> {
        let value = self.value.read().await;
        if let Value::Pending(pending) = &*value {
            return pending.clone();
        }

        drop(value);

        let mut value = self.value.write().await;

        *value = Value::Pending(match value.clone() {
            Value::Pending(pending) => pending,
            Value::Provider(provider) => provider.provide(inject).shared(),
        });

        if let Value::Pending(pending) = &*value {
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
        let available = self.get_available_keys().await;

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
        &self,
        key: Key,
    ) -> Result<Option<Arc<T>>> {
        if let Some(injector) = self.0.read().await.get(&key) {
            let pending = injector.request(self.clone()).await;
            let value = pending.await?;

            return value
                .downcast::<T>()
                .map(Some)
                .map_err(|_err| Error::TypeMismatch(key));
        }

        Ok(None)
    }

    /// Provide a dependency directly
    pub(crate) async fn inject_key<T: Any + Send + Sync>(&self, key: Key, dep: T) -> Result<()> {
        match self.0.write().await.entry(key.clone()) {
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
        let available = self.get_available_keys().await;

        match self.0.write().await.entry(key.clone()) {
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
                available,
            }),
        }
    }

    pub(crate) async fn provide_key(
        &self,
        key: Key,
        provider: impl Provider + 'static,
    ) -> Result<()> {
        match self.0.write().await.entry(key.clone()) {
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
        provider: impl Provider + 'static,
    ) -> Result<()> {
        let available = self.get_available_keys().await;

        match self.0.write().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let _ = entry.insert(Injector::from_provider(provider));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available,
            }),
        }
    }

    pub(crate) async fn consume_key<T: Any + Send + Sync>(&self, key: Key) -> Result<T> {
        let available = self.get_available_keys().await;

        self.consume_key_opt(key.clone())
            .await?
            .ok_or(Error::NotFound {
                missing: key,
                available,
            })
    }

    pub(crate) async fn consume_key_opt<T: Any + Send + Sync>(
        &self,
        key: Key,
    ) -> Result<Option<T>> {
        if let Some(dep) = self.get_key_opt::<T>(key.clone()).await? {
            // Since we have a reference to the dependency, we can remove it from the container and
            // drop the reference it holds
            self.remove_key(key.clone()).await?;

            // Now we can try to unwrap the Arc, but if there is more than 1 strong pointer, this
            // will fail and the CannotConsume error will be returned
            return Arc::try_unwrap(dep)
                .map(Some)
                .map_err(|arc| Error::CannotConsume {
                    key,
                    strong_count: Arc::strong_count(&arc),
                });
        };

        Ok(None)
    }

    pub(crate) async fn remove_key(&self, key: Key) -> Result<()> {
        let available = self.get_available_keys().await;

        match self.0.write().await.entry(key.clone()) {
            Entry::Occupied(entry) => {
                let _ = entry.remove();

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available,
            }),
        }
    }

    async fn get_available_keys(&self) -> Vec<Key> {
        let container = self.0.read().await;

        container.keys().cloned().collect()
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
    use std::sync::Arc;

    use super::*;

    // Mock Dependencies
    // -----------------

    pub trait HasId: Send + Sync {
        fn get_id(&self) -> String;
    }

    #[derive(Debug, Clone)]
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
    impl Provider for TestServiceProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<Dependency>> {
            // Attempt to retrieve a dependency inside the Provider to test for deadlocks
            i.get_type_opt::<String>().await?;

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
    impl Provider for OtherServiceProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<Dependency>> {
            i.get_type_opt::<String>().await?;

            Ok(Arc::new(OtherService::new(self.id.clone())))
        }
    }

    #[derive(Default)]
    pub struct HasIdProvider {}

    #[async_trait]
    impl Provider for HasIdProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<Dependency>> {
            i.get_type_opt::<String>().await?;

            let dep: Box<dyn HasId> = Box::new(OtherService::new("test-service".to_string()));

            Ok(Arc::new(dep))
        }
    }
}
