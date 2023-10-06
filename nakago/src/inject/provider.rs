use std::{any::Any, collections::hash_map::Entry, sync::Arc};

use async_trait::async_trait;

use super::{
    injector::{Dependency, Injector},
    Error, Inject, Key, Result,
};

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<T: ?Sized>: Send + Sync {
    /// Provide a dependency for the container
    async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<T>>;
}

impl Inject {
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

    use crate::inject::container::test::{HasId, OtherService, TestService};

    use super::*;

    // Mock Providers
    // -----------------

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

            Ok(Arc::new(Box::new(OtherService::new(
                "test-service".to_string(),
            ))))
        }
    }
}
