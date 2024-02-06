use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use cqrs_es::{persist::ViewRepository, Aggregate, View};
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;
use postgres_es::PostgresViewRepository;

use crate::postgres::POSTGRES_POOL;

/// Provide the default Postgres Repository for Character Views
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Provide<V, A> {
    name: String,
    _phantom: std::marker::PhantomData<(V, A)>,
}

impl<V, A> Provide<V, A> {
    /// Create a new Provide instance
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
#[Provider]
impl<V, A> Provider<Box<dyn ViewRepository<V, A>>> for Provide<V, A>
where
    V: Send + Sync + Any + View<A>,
    A: Send + Sync + Any + Aggregate,
{
    async fn provide(
        self: Arc<Self>,
        i: Inject,
    ) -> provider::Result<Arc<Box<dyn ViewRepository<V, A>>>> {
        let pool = i.get(&POSTGRES_POOL).await?;

        Ok(Arc::new(Box::new(PostgresViewRepository::new(
            &self.name,
            (*pool).clone(),
        ))))
    }
}
