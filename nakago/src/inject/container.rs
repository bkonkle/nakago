use std::{
    any::Any,
    collections::HashMap,
    future::{ready, Future},
    mem,
    pin::Pin,
    sync::Arc,
};

use futures::{future::Shared, FutureExt};
use tokio::sync::Mutex;

use super::{Error, Key, Result};

pub type Dependency = dyn Any + Send + Sync;
pub type Pending = dyn Future<Output = Result<Arc<Dependency>>>;
pub type Provider = dyn FnOnce(&Inject) -> Pin<Box<Pending>>;

enum Injector {
    Pending(Shared<Pin<Box<Pending>>>),
    Provider(Option<Box<Provider>>),
}

/// The injection Container
#[derive(Default)]
pub struct Inject(pub(crate) HashMap<Key, Mutex<Injector>>);

// The base methods powering both the Tag and TypeId modes
impl Inject {
    /// Retrieve a reference to a dependency if it exists, and return an error otherwise
    pub(crate) async fn get_key<T: Any + Send + Sync>(&self, key: Key) -> Result<Arc<T>> {
        if let Some(injector) = self.0.get(&key) {
            let injector = &mut *injector.lock().await;

            let transformed = match injector {
                Injector::Pending(pending) => Injector::Pending(pending.clone()),
                Injector::Provider(provider) => {
                    Injector::Pending((provider.take().unwrap())(self).shared())
                }
            };
            mem::replace(injector, transformed);

            let pending = match injector {
                Injector::Pending(pending) => pending,
                Injector::Provider(_) => unreachable!(),
            };

            return pending
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
        let injector = self
            .0
            .remove(&key)
            .ok_or_else(|| Error::NotFound {
                missing: key.clone(),
                available: self.available_type_names(),
            })?
            .into_inner();

        match injector {
            Injector::Pending(pending) => {
                let value = pending.await?;
                let arc = value
                    .downcast::<T>()
                    .map_err(|_| Error::TypeMismatch(key.clone()))?;

                return Arc::try_unwrap(arc).map_err(|_| Error::TypeMismatch(key));
            }
            Injector::Provider(_) => unreachable!(),
        }
    }

    /// Provide a dependency directly
    pub(crate) fn inject_key<T: Any + Send + Sync>(&mut self, key: Key, dep: T) -> Result<()> {
        if self.0.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        let pending: Pin<Box<Pending>> =
            Box::pin(ready::<Result<Arc<Dependency>>>(Ok(Arc::new(dep))));

        self.0
            .insert(key, Mutex::new(Injector::Pending(pending.shared())));

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

        let pending: Pin<Box<Pending>> =
            Box::pin(ready::<Result<Arc<Dependency>>>(Ok(Arc::new(dep))));

        self.0
            .insert(key, Mutex::new(Injector::Pending(pending.shared())));

        Ok(())
    }

    /// Use a Provider function to inject a dependency.
    pub fn provide_key<P>(&mut self, key: Key, provider: P) -> Result<()>
    where
        P: FnOnce(&Inject) -> Pin<Box<Pending>>,
    {
        if self.0.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        self.0.insert(
            key,
            Mutex::new(Injector::Provider(Some(Box::new(provider)))),
        );

        Ok(())
    }

    /// Use a Provider function to replace an existing dependency.
    pub async fn replace_key_with<P>(&mut self, key: Key, provider: P) -> Result<()>
    where
        P: FnOnce(&Inject) -> Pin<Box<Pending>>,
    {
        if !self.0.contains_key(&key) {
            return Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            });
        }

        return self.provide_key(key, provider);
    }

    /// Return a list of all available type names in the map
    pub(crate) fn available_type_names(&self) -> Vec<Key> {
        self.0.keys().cloned().collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
    use std::{any::type_name, sync::Arc};
    use tokio::time::{sleep, Duration};

    use crate::inject::{
        tag::test::{DYN_TAG, OTHER_TAG, SERVICE_TAG},
        Key,
    };

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
    ) -> impl FnOnce(&Inject) -> Pin<Box<dyn Future<Output = Result<Arc<dyn HasId>>> + 'a>> {
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
