use async_trait::async_trait;
use futures::{future::Shared, Future, FutureExt};
use std::{pin::Pin, sync::Arc};

use super::{Error, Inject, Result};

/// A trait for async injection Providers
#[async_trait]
pub trait Provider<T> {
    /// Provide a dependency for the container
    async fn provide(&self, i: &Inject) -> Result<Arc<T>>;
}

enum Value<T> {
    Provider(Box<dyn Provider<T>>),
    Pending(Shared<Pin<Box<dyn Future<Output = Result<Arc<T>>> + Send>>>),
}

// An Injector is a wrapper around a Dependency that can be in one of two states:
//   - Pending: The Dependency has been requested, and is wrapped in a Shared Promise that will
//     resolve to the Dependency when it is ready.
//   - Provider: The Dependency has not been requested yet, and a Provider function is available
//     to create the Dependency when it is requested.
pub(crate) struct Injector<T> {
    value: Value<T>,
}

impl<T> Injector<T> {
    pub(crate) fn from_pending(
        pending: Shared<Pin<Box<dyn Future<Output = Result<Arc<T>>> + Send>>>,
    ) -> Self {
        Self {
            value: Value::Pending(pending),
        }
    }

    pub(crate) fn from_provider(provider: Box<dyn Provider<T>>) -> Self {
        Self {
            value: Value::Provider(provider),
        }
    }

    pub(crate) fn request(
        &self,
        inject: &Inject,
    ) -> Shared<Pin<Box<dyn Future<Output = Result<Arc<T>>> + Send>>> {
        let pending = match self.value {
            // If this Dependency hasn't been requested yet, kick off the inner Shared Promise,
            // which is a Provider that will resolve the Promise with the Dependency inside
            // an Arc.
            Value::Provider(provider) => provider.provide(inject).shared(),
            // If this is a Dependency that has already been requested, it will already be in a
            // Pending state. In that cose, clone the inner Shared Promise (which clones the
            // inner Arc around the Dependency at the time it's resolved).
            Value::Pending(pending) => pending,
        };

        self.value = Value::Pending(pending.clone());

        pending
    }
}

/// Wrap an error that can be converted into an Anyhow error with an inject Provider error
pub fn to_provider_error<E>(e: E) -> Error
where
    anyhow::Error: From<E>,
{
    Error::Provider(Arc::new(e.into()))
}
