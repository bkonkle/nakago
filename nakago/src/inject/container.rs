use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    future::ready,
    pin::Pin,
    sync::Arc,
};

use backtrace::Backtrace;
use futures::{Future, FutureExt};
use tokio::sync::RwLock;

use super::{Dependency, Error, Injector, Key, Result};

/// A Dependency Injection container based on the concept of Shared Futures, which multiple
/// independent threads can await. The container holds a map of Keys to Injectors, and provides
/// methods for retrieving, injecting, and removing Dependencies and Providers.
#[derive(Default, Clone)]
pub struct Inject(pub(crate) Arc<RwLock<HashMap<Key, Injector>>>);

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
                backtrace: Arc::new(Backtrace::new()),
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
                backtrace: Arc::new(Backtrace::new()),
            })
        }
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
                backtrace: Arc::new(Backtrace::new()),
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

    /// Temporarily remove a dependency from the container and try to unwrap it from the Arc, which
    /// will only succeed if there are no other strong pointers to the value. Then, apply a function
    /// to it, and then injects it back into the container.
    pub async fn modify_key<T, F>(&self, key: Key, modify: F) -> Result<()>
    where
        T: Any + Send + Sync,
        F: FnOnce(T) -> Result<T>,
    {
        if let Some(dep) = self.get_key_opt::<T>(key.clone()).await? {
            // Remove the dependency from the container and drop the reference it holds
            self.remove_key(key.clone()).await?;

            // If there is more than 1 strong pointer, this will fail and the CannotConsume error
            // will be returned
            let dep = Arc::try_unwrap(dep)
                .map(Some)
                .map_err(|arc| Error::CannotConsume {
                    key: key.clone(),
                    strong_count: Arc::strong_count(&arc),
                })?;

            if let Some(dep) = dep {
                self.inject_key(key.clone(), modify(dep)?).await?;
            }

            return Ok(());
        };

        Err(Error::NotFound {
            missing: key,
            available: self.get_available_keys().await,
            backtrace: Arc::new(Backtrace::new()),
        })
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
                backtrace: Arc::new(Backtrace::new()),
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
                backtrace: Arc::new(Backtrace::new()),
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

#[cfg(test)]
pub(crate) mod test {
    // Mock Dependencies
    // -----------------

    use derive_new::new;

    pub trait HasId: Send + Sync {
        fn get_id(&self) -> String;
    }

    #[derive(Debug, Clone, new)]
    pub struct TestService {
        pub(crate) id: String,
    }

    impl HasId for TestService {
        fn get_id(&self) -> String {
            self.id.clone()
        }
    }

    #[derive(new)]
    pub struct OtherService {
        pub(crate) other_id: String,
    }

    impl HasId for OtherService {
        fn get_id(&self) -> String {
            self.other_id.clone()
        }
    }
}
