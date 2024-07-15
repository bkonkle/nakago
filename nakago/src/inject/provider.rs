use std::{any::Any, collections::hash_map::Entry, sync::Arc};

use async_trait::async_trait;
use backtrace::Backtrace;
use thiserror::Error;

use super::{errors, Dependency, Inject, Injector, Key};

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
    ) -> errors::Result<()> {
        match self.0.write().await.entry(key.clone()) {
            Entry::Occupied(_) => Err(super::Error::Occupied(key)),
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
    ) -> errors::Result<()> {
        let available = self.get_available_keys().await;

        match self.0.write().await.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                let _ = entry.insert(Injector::from_provider::<T>(provider));

                Ok(())
            }
            Entry::Vacant(_) => Err(super::Error::NotFound {
                missing: key,
                available,
                backtrace: Arc::new(Backtrace::new()),
            }),
        }
    }
}

/// A Provider Result
pub type Result<T> = std::result::Result<T, Error>;

/// Provider Errors
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// A generic error thrown from a Provider
    #[error("provider failure")]
    Any(#[from] Arc<anyhow::Error>),

    /// An injection error thrown from a Provider
    #[error("injection failure")]
    Inject(#[from] errors::Error),
}

/// Wrap an error that can be converted into an Anyhow error with a Provider error
pub fn to_provider_error<E>(e: E) -> Error
where
    anyhow::Error: From<E>,
{
    Error::Any(Arc::new(e.into()))
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Arc;

    use derive_new::new;
    use nakago_derive::Provider;

    use crate::inject::container::test::{HasId, OtherService, TestService};

    use super::*;

    // Mock Providers
    // -----------------

    #[derive(new)]
    pub struct TestServiceProvider {
        id: String,
    }

    #[Provider(internal)]
    #[async_trait]
    impl Provider<TestService> for TestServiceProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<TestService>> {
            // Attempt to retrieve a dependency inside the Provider to test for deadlocks
            i.get_opt::<String>().await?;

            Ok(Arc::new(TestService::new(self.id.clone())))
        }
    }

    #[derive(new)]
    pub struct OtherServiceProvider {
        id: String,
    }

    #[Provider(internal)]
    #[async_trait]
    impl Provider<OtherService> for OtherServiceProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<OtherService>> {
            i.get_opt::<String>().await?;

            Ok(Arc::new(OtherService::new(self.id.clone())))
        }
    }

    #[derive(Default)]
    pub struct HasIdProvider {}

    #[Provider(internal)]
    #[async_trait]
    impl Provider<Box<dyn HasId>> for HasIdProvider {
        async fn provide(self: Arc<Self>, i: Inject) -> Result<Arc<Box<dyn HasId>>> {
            i.get_opt::<String>().await?;

            Ok(Arc::new(Box::new(OtherService::new(
                "test-service".to_string(),
            ))))
        }
    }
}
