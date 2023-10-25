use std::{any::Any, pin::Pin, sync::Arc};

use futures::{future::Shared, Future, FutureExt};
use tokio::sync::RwLock;

use super::{Provider, Result};

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

impl Injector {
    // Create a new Injector from a value that is already Pending
    pub(crate) fn from_pending(pending: Shared<Pending>) -> Self {
        Self {
            value: RwLock::new(Value::Pending(pending)),
        }
    }

    // Create a new Injector from a Provider
    pub(crate) fn from_provider<T: Any + Send + Sync>(
        provider: impl Provider<T> + Provider<Dependency> + 'static,
    ) -> Self {
        Self {
            value: RwLock::new(Value::Provider(Arc::new(provider))),
        }
    }

    // Request a Pending Future from the Injector. If the value is a Provider, it will be
    // replaced with a Pending Future that will resolve to the provided Dependency.
    pub(crate) async fn request(&self, inject: crate::Inject) -> Shared<Pending> {
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
