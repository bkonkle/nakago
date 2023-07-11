use std::{
    any::Any,
    collections::HashMap,
    future::{ready, Future},
    pin::Pin,
    sync::Arc,
};

use futures::{future::Shared, FutureExt};

use super::{Error, Key, Result};

pub type Dependency = dyn Any + Send + Sync;
pub type Pending = dyn Future<Output = Result<Arc<Dependency>>>;
pub type Provider = dyn FnOnce(&Inject) -> Pin<Box<Pending>>;

enum Value {
    Pending(Shared<Pin<Box<Pending>>>),
    Provider(Box<Provider>),
}

// An Injector is a wrapper around a Dependency that can be in one of two states:
//   - Pending: The Dependency has been requested, and is wrapped in a Shared Promise that will
//     resolve to the Dependency when it is ready.
//   - Provider: The Dependency has not been requested yet, and a Provider function is available
//     to create the Dependency when it is requested.
pub(crate) struct Injector {
    value: Value,
}

impl Injector {
    fn from_pending(pending: Shared<Pin<Box<Pending>>>) -> Self {
        Self {
            value: Value::Pending(pending),
        }
    }

    pub(crate) fn from_provider(provider: Box<Provider>) -> Self {
        Self {
            value: Value::Provider(provider),
        }
    }

    fn request(&self, inject: &Inject) -> Shared<Pin<Box<Pending>>> {
        let pending = match self.value {
            // If this is a Dependency that has already been requested, it will already be in a
            // Pending state. In that cose, clone the inner Shared Promise (which clones the
            // inner Arc around the Dependency at the time it's resolved).
            Value::Pending(pending) => pending,
            // If this Dependency hasn't been requested yet, kick off the inner Shared Promise,
            // which is a Provider that will resolve the Promise with the Dependency inside
            // an Arc.
            Value::Provider(provider) => provider(inject).shared(),
        };

        self.value = Value::Pending(pending.clone());

        pending
    }
}

/// The injection Container
#[derive(Default)]
pub struct Inject {
    pub(crate) container: HashMap<Key, Injector>,
}

// The base methods powering both the Tag and TypeId modes
impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub(crate) async fn get_key<T: Any + Send + Sync>(&self, key: Key) -> Result<Arc<T>> {
        if let Some(injector) = self.container.get(&key) {
            return injector
                .request(self)
                .await
                .and_then(|value| value.downcast::<T>().map_err(|_| Error::TypeMismatch(key)));
        }

        Err(Error::NotFound {
            missing: key.clone(),
            available: self.available_type_names(),
        })
    }

    /// Remove a dependency from the map and return it for use
    pub(crate) async fn consume_key<T: Any + Send + Sync>(&mut self, key: Key) -> Result<T> {
        let injector = self.container.remove(&key).ok_or_else(|| Error::NotFound {
            missing: key.clone(),
            available: self.available_type_names(),
        })?;

        let value = injector.request(self).await?;
        let arc = value
            .downcast::<T>()
            .map_err(|_| Error::TypeMismatch(key.clone()))?;

        return Arc::try_unwrap(arc).map_err(|_| Error::TypeMismatch(key));
    }

    /// Provide a dependency directly
    pub(crate) fn inject_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if self.container.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let pending: Pin<Box<Pending>> =
            Box::pin(ready::<Result<Arc<Dependency>>>(Ok(Arc::new(dep))));

        self.container
            .insert(key, Injector::from_pending(pending.shared()));

        Ok(())
    }

    /// Replace an existing dependency directly
    pub(crate) fn replace_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if !self.container.contains_key(&key) {
            return Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            });
        }

        let pending: Pin<Box<Pending>> =
            Box::pin(ready::<Result<Arc<Dependency>>>(Ok(Arc::new(dep))));

        self.container
            .insert(key, Injector::from_pending(pending.shared()));

        Ok(())
    }

    /// Use a Provider function to inject a dependency.
    pub fn provide_key<P>(&mut self, key: Key, provider: P) -> Result<()>
    where
        P: FnOnce(&Inject) -> Pin<Box<Pending>>,
    {
        if self.container.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        self.container
            .insert(key, Injector::from_provider(Box::new(provider)));

        Ok(())
    }

    /// Use a Provider function to replace an existing dependency.
    pub async fn replace_key_with<P>(&mut self, key: Key, provider: P) -> Result<()>
    where
        P: FnOnce(&Inject) -> Pin<Box<Pending>>,
    {
        if !self.container.contains_key(&key) {
            return Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            });
        }

        return self.provide_key(key, provider);
    }

    /// Return a list of all available type names in the map
    pub(crate) fn available_type_names(&self) -> Vec<Key> {
        self.container.keys().cloned().collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

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

    fn provide_test_service(
        id: String,
    ) -> impl FnOnce(&Inject) -> Pin<Box<dyn Future<Output = Result<Arc<TestService>>>>> {
        move |i| Box::pin(async move { Ok(Arc::new(TestService::new(id))) })
    }

    fn provide_other_service(
        id: String,
    ) -> impl FnOnce(&Inject) -> Pin<Box<dyn Future<Output = Result<Arc<OtherService>>>>> {
        move |i| Box::pin(async move { Ok(Arc::new(OtherService::new(id))) })
    }

    fn provide_dyn_has_id<'a>(
    ) -> impl FnOnce(&'a Inject) -> Pin<Box<dyn Future<Output = Result<Arc<dyn HasId>>> + 'a>> {
        move |i| {
            Box::pin(async move {
                // Trigger a borrow so that the reference to `Inject` has to be held across the await
                // point below, to test issues with Inject thread safety.
                let _ = i.get_type::<String>();

                sleep(Duration::from_millis(1)).await;

                let arc: Arc<dyn HasId> = Arc::new(OtherService::new("test-service".to_string()));

                Ok(arc)
            })
        }
    }
}
