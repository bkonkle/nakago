use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    future::{ready, Future},
    pin::Pin,
    sync::Arc,
};

use futures::{future::Shared, FutureExt};

use super::{Error, Key, Result};

/// The injection Container
#[derive(Default)]
pub struct Inject<'a> {
    pub(crate) container: HashMap<Key, Injector<'a>>,
}

// An Injector is a wrapper around a Dependency that can be in one of two states:
//   - Pending: The Dependency has been requested, and is wrapped in a Shared Promise that will
//     resolve to the Dependency when it is ready.
//   - Provider: The Dependency has not been requested yet, and a Provider function is available
//     to create the Dependency when it is requested.
pub(crate) struct Injector<'a> {
    value: Value<'a>,
}

enum Value<'a> {
    Pending(Shared<Pending<'a>>),
    Provider(Box<Provider<'a>>),
}

pub type Dependency = dyn Any + Send + Sync;
pub type Pending<'a> = Pin<Box<dyn Future<Output = Result<Arc<Dependency>>> + 'a>>;
pub type Provider<'a> = dyn FnOnce(&'a Inject<'a>) -> Pending<'a>;

impl<'a> Injector<'a> {
    fn from_pending(pending: Shared<Pending<'a>>) -> Self {
        Self {
            value: Value::Pending(pending),
        }
    }

    pub(crate) fn from_provider(provider: Box<Provider<'a>>) -> Self {
        Self {
            value: Value::Provider(provider),
        }
    }

    fn request(&self, inject: &'a Inject<'a>) -> Shared<Pending<'a>> {
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

// The base methods powering both the Tag and TypeId modes
impl<'a> Inject<'a> {
    pub(crate) async fn get_key<T: Any + Send + Sync>(&'a self, key: Key) -> Result<Arc<T>> {
        if let Some(dep) = self.get_key_opt::<T>(key.clone()).await? {
            Ok(dep)
        } else {
            Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            })
        }
    }

    pub(crate) async fn get_key_opt<T: Any + Send + Sync>(
        &'a self,
        key: Key,
    ) -> Result<Option<Arc<T>>> {
        let injector = self.container.get(&key);

        if let Some(injector) = injector {
            let value = injector.request(self).await?;

            return value
                .downcast::<T>()
                .map(|value| Some(value))
                .map_err(|_| Error::TypeMismatch(key));
        }

        Ok(None)
    }

    pub(crate) async fn consume_key<T: Any + Send + Sync>(&'a mut self, key: Key) -> Result<T> {
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

    pub(crate) fn inject_key<T: Any + Send + Sync>(&'a mut self, key: Key, dep: T) -> Result<()> {
        match self.container.entry(key) {
            Entry::Occupied(_) => Err(Error::Occupied(key)),
            Entry::Vacant(entry) => {
                let pending: Pending<'a> =
                    ready::<Result<Arc<Dependency>>>(Ok(Arc::new(dep))).boxed();

                let _ = entry.insert(Injector::from_pending(pending.shared()));

                Ok(())
            }
        }
    }

    pub(crate) fn replace_key<T: Any + Send + Sync>(&'a mut self, key: Key, dep: T) -> Result<()> {
        match self.container.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let pending: Pending<'a> =
                    ready::<Result<Arc<Dependency>>>(Ok(Arc::new(dep))).boxed();

                let _ = entry.insert(Injector::from_pending(pending.shared()));

                Ok(())
            }
            Entry::Vacant(_) => Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            }),
        }
    }

    pub(crate) fn provide_key<P>(&'a mut self, key: Key, provider: P) -> Result<()>
    where
        P: FnOnce(&'a Inject<'a>) -> Pending<'a>,
    {
        if self.container.contains_key(&key) {
            return Err(Error::Occupied(key));
        }

        self.container
            .insert(key, Injector::from_provider(Box::new(provider)));

        Ok(())
    }

    pub(crate) fn replace_key_with<P>(&'a mut self, key: Key, provider: P) -> Result<()>
    where
        P: FnOnce(&'a Inject<'a>) -> Pending<'a>,
    {
        if !self.container.contains_key(&key) {
            return Err(Error::NotFound {
                missing: key,
                available: self.available_type_names(),
            });
        }

        return self.provide_key(key, provider);
    }

    pub(crate) fn available_type_names(&'a self) -> Vec<Key> {
        self.container.keys().cloned().collect()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use fake::Fake;
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

    fn provide_test_service(id: String) -> impl for<'a> FnOnce(&'a Inject<'a>) -> Pending<'a> {
        move |i| {
            async move {
                let dependency: Arc<Dependency> = Arc::new(TestService::new(id));

                Ok(dependency)
            }
            .boxed()
        }
    }

    fn provide_other_service(id: String) -> impl for<'a> FnOnce(&'a Inject<'a>) -> Pending<'a> {
        move |i| {
            async move {
                let dependency: Arc<Dependency> = Arc::new(OtherService::new(id));

                Ok(dependency)
            }
            .boxed()
        }
    }

    fn provide_dyn_has_id() -> impl for<'a> FnOnce(&'a Inject<'a>) -> Pending<'a> {
        move |i| {
            Box::pin(async move {
                // Trigger a borrow so that the reference to `Inject` has to be held across the await
                // point below, to test issues with Inject thread safety.
                let _ = i.get_type::<String>().await?;

                sleep(Duration::from_millis(1)).await;

                let arc: Arc<Dependency> = Arc::new(OtherService::new("test-service".to_string()));

                Ok(arc)
            })
        }
    }

    #[tokio::test]
    async fn test_provide_success() -> Result<()> {
        let mut i = Inject::default();

        i.provide_type::<Arc<TestService>, _>(provide_test_service(fake::uuid::UUIDv4.fake()))?;

        assert!(
            i.container
                .contains_key(&Key::from_type_id::<Arc<TestService>>()),
            "key does not exist in injection container"
        );

        let _ = i.get_type::<Box<TestService>>().await?;

        Ok(())
    }
}
