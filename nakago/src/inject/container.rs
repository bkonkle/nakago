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

// An Injector holds a locked value that can be either a Provider or a Pending Future. The
// Injector is responsible for providing a Pending Future to the container when requested, and
// updating the value to a Pending Shared Future if it is a Provider.
pub(crate) struct Injector {
    value: RwLock<Value>,
}

// The value of an Injector can be either a Provider or a Pending Shared Future.
#[derive(Clone)]
enum Value {
    Provider(Arc<dyn Provider<Dependency>>),
    Pending(Shared<Pending>),
}

/// A Dependency that can be injected into the container
pub type Dependency = dyn Any + Send + Sync;

/// A Future that will resolve to a Dependency
pub type Pending = Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>>;

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<T: ?Sized>: Send + Sync {
    /// Provide a dependency for the container
    async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<T>>;
}

impl Injector {
    // Create a new Injector from a value that is already Pending
    fn from_pending(pending: Shared<Pending>) -> Self {
        Self {
            value: RwLock::new(Value::Pending(pending)),
        }
    }

    // Create a new Injector from a Provider
    fn from_provider<T: Any + Send + Sync>(
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Self {
        Self {
            value: RwLock::new(Value::Provider(Arc::new(provider))),
        }
    }

    // Request a Pending Future from the Injector. If the value is a Provider, it will be
    // replaced with a Pending Future that will resolve to the provided Dependency.
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

// The Inject container is responsible for providing access to Dependencies and Providers. It
// holds a map of Keys to Injectors, and provides methods for retrieving, injecting, and removing
// Dependencies and Providers. This base implementation provides general methods that are used by
// the Tag and TypeId specific interfaces.
impl Inject {
    /// Retrieve a reference to a Dependency if it exists. Return a NotFound error if the Key isn't
    /// present.
    pub async fn get_key<T: Any + Send + Sync>(&self, key: Key) -> Result<Arc<T>> {
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

    /// Retrieve a reference to a Dependency if it exists.
    pub async fn get_key_opt<T: Any + Send + Sync>(&self, key: Key) -> Result<Option<Arc<T>>> {
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

    /// Override an existing Dependency directly, using core::future::ready to wrap it in an
    /// immediately resolving Pending Future. Return true if the Key was already present.
    pub async fn override_key<T: Any + Send + Sync>(&self, key: Key, dep: T) -> Result<bool> {
        match self.0.write().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let pending: Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>> =
                    Box::pin(ready::<Result<Arc<dyn Any + Send + Sync>>>(Ok(Arc::new(
                        dep,
                    ))));

                let _ = entry.insert(Injector::from_pending(pending.shared()));

                Ok(true)
            }
            Entry::Vacant(entry) => {
                let pending: Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + Send>> =
                    Box::pin(ready::<Result<Arc<dyn Any + Send + Sync>>>(Ok(Arc::new(
                        dep,
                    ))));

                let _ = entry.insert(Injector::from_pending(pending.shared()));

                Ok(false)
            }
        }
    }

    /// Provide a Dependency directly, using core::future::ready to wrap it in an immediately
    /// resolving Pending Future.
    pub async fn inject_key<T: Any + Send + Sync>(&self, key: Key, dep: T) -> Result<()> {
        if self.0.read().await.contains_key(&key) {
            Err(Error::Occupied(key))
        } else {
            let _ = self.override_key(key, dep).await?;

            Ok(())
        }
    }

    /// Replace an existing Dependency directly, using core::future::ready to wrap it in an
    /// immediately resolving Pending Future. Return a NotFound error if the Key isn't present.
    pub async fn replace_key<T: Any + Send + Sync>(&self, key: Key, dep: T) -> Result<()> {
        let available = self.get_available_keys().await;

        if self.0.read().await.contains_key(&key) {
            let _ = self.override_key(key, dep).await?;

            Ok(())
        } else {
            Err(Error::NotFound {
                missing: key,
                available,
            })
        }
    }

    /// Inject a Dependency Provider
    pub async fn provide_key<T: Any + Send + Sync>(
        &self,
        key: Key,
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Result<()> {
        match self.0.write().await.entry(key.clone()) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let _ = entry.insert(Injector::from_provider::<T>(provider));

                Ok(())
            }
        }
    }

    /// Inject a replacement Dependency Provider if the Key is present
    pub async fn replace_key_with<T: Any + Send + Sync>(
        &self,
        key: Key,
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Result<()> {
        let available = self.get_available_keys().await;

        match self.0.write().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let _ = entry.insert(Injector::from_provider::<T>(provider));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available,
            }),
        }
    }

    /// Remove a Dependency from the container and try to unwrap it from the Arc, which will only
    /// succeed if there are no other strong pointers to the value. Any Arcs handed out will still
    /// be valid, but the container will no longer hold a reference. Return a NotFound error if the
    /// Key isn't present.
    pub async fn consume_key<T: Any + Send + Sync>(&self, key: Key) -> Result<T> {
        let available = self.get_available_keys().await;

        self.consume_key_opt(key.clone())
            .await?
            .ok_or(Error::NotFound {
                missing: key,
                available,
            })
    }

    /// Remove a Dependency from the container and try to unwrap it from the Arc, which will only
    /// succeed if there are no other strong pointers to the value. Any Arcs handed out will still
    /// be valid, but the container will no longer hold a reference.
    pub async fn consume_key_opt<T: Any + Send + Sync>(&self, key: Key) -> Result<Option<T>> {
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

    /// Discard a Dependency from the container. Any Arcs handed out will still be valid, but
    /// the container will no longer hold a reference.
    pub async fn remove_key(&self, key: Key) -> Result<()> {
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

    /// Destroy the container and discard all Dependencies except for the given Key. Any Arcs handed
    /// out will still be valid, but the container will be fully unloaded and all references will be
    /// dropped. Return a NotFound error if the Key isn't present.
    pub async fn eject_key<T: Any + Send + Sync>(self, key: Key) -> Result<T> {
        let available = self.get_available_keys().await;

        self.eject_key_opt(key.clone())
            .await?
            .ok_or(Error::NotFound {
                missing: key,
                available,
            })
    }

    /// Destroy the container and discard all Dependencies except for the given Key. Any Arcs handed
    /// out will still be valid, but the container will be fully unloaded and all references will be
    /// dropped.
    pub async fn eject_key_opt<T: Any + Send + Sync>(self, key: Key) -> Result<Option<T>> {
        // First get the Dependency
        let dep = self.get_key_opt::<T>(key.clone()).await?;

        // Then drop the container
        drop(self);

        if let Some(value) = dep {
            // Now we can try to unwrap the Arc, but if there is more than 1 strong pointer, this
            // will fail and the CannotConsume error will be returned
            return Arc::try_unwrap(value)
                .map(Some)
                .map_err(|arc| Error::CannotConsume {
                    key,
                    strong_count: Arc::strong_count(&arc),
                });
        }

        Ok(None)
    }

    /// Get all available Keys in the container.
    pub async fn get_available_keys(&self) -> Vec<Key> {
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

    use nakago_derive::Provider;

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

    #[Provider(internal)]
    #[async_trait]
    impl Provider<TestService> for TestServiceProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<TestService>> {
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

    #[Provider(internal)]
    #[async_trait]
    impl Provider<OtherService> for OtherServiceProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<OtherService>> {
            i.get_type_opt::<String>().await?;

            Ok(Arc::new(OtherService::new(self.id.clone())))
        }
    }

    #[derive(Default)]
    pub struct HasIdProvider {}

    #[Provider(internal)]
    #[async_trait]
    impl Provider<Box<dyn HasId>> for HasIdProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<Box<dyn HasId>>> {
            i.get_type_opt::<String>().await?;

            let dep: Box<dyn HasId> = Box::new(OtherService::new("test-service".to_string()));

            Ok(Arc::new(dep))
        }
    }
}
