use std::{any::Any, marker::PhantomData, sync::Arc};

use async_graphql::{ObjectType, Schema, SchemaBuilder, SubscriptionType};
use async_trait::async_trait;
use nakago::{inject, Hook, Inject, Provider, Tag};
use nakago_derive::Provider;

/// Provides a SchemaBuilder that can be modified by hooks to initialize the GraphQL Schema
///
/// **Provides:** `SchemaBuilder<Query, Mutation, Subscription>`
#[derive(Default)]
pub struct ProvideBuilder<Query: Any, Mutation: Any, Subscription: Any> {
    _query: PhantomData<Query>,
    _mutation: PhantomData<Mutation>,
    _subscription: PhantomData<Subscription>,
}

#[Provider]
#[async_trait]
impl<Query, Mutation, Subscription> Provider<SchemaBuilder<Query, Mutation, Subscription>>
    for ProvideBuilder<Query, Mutation, Subscription>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    async fn provide(
        self: Arc<Self>,
        _i: Inject,
    ) -> inject::Result<Arc<SchemaBuilder<Query, Mutation, Subscription>>> {
        Ok(Arc::new(Schema::build(
            Query::default(),
            Mutation::default(),
            Subscription::default(),
        )))
    }
}

/// Initialize the GraphQL Schema and inject it with the given Tag
#[derive(Default)]
pub struct Init<Query: Any, Mutation: Any, Subscription: Any> {
    builder_tag: Option<&'static Tag<SchemaBuilder<Query, Mutation, Subscription>>>,
    schema_tag: Option<&'static Tag<Schema<Query, Mutation, Subscription>>>,
}

impl<Query: Any, Mutation: Any, Subscription: Any> Init<Query, Mutation, Subscription> {
    /// Add a builder tag to the hook
    pub fn with_builder_tag(
        self,
        tag: &'static Tag<SchemaBuilder<Query, Mutation, Subscription>>,
    ) -> Self {
        Self {
            builder_tag: Some(tag),
            ..self
        }
    }

    /// Add a schema tag to the hook
    pub fn with_schema_tag(self, tag: &'static Tag<Schema<Query, Mutation, Subscription>>) -> Self {
        Self {
            schema_tag: Some(tag),
            ..self
        }
    }
}

#[async_trait]
impl<Query, Mutation, Subscription> Hook for Init<Query, Mutation, Subscription>
where
    Query: ObjectType + Default + 'static,
    Mutation: ObjectType + Default + 'static,
    Subscription: SubscriptionType + Default + 'static,
{
    async fn handle(&self, inject: Inject) -> inject::Result<()> {
        let schema_builder = if let Some(tag) = self.builder_tag {
            inject.consume(tag).await?
        } else {
            inject
                .consume_type::<SchemaBuilder<Query, Mutation, Subscription>>()
                .await?
        };

        let schema = schema_builder.finish();

        if let Some(tag) = self.schema_tag {
            inject.inject(tag, schema).await?;
        } else {
            inject.inject_type(schema).await?;
        }

        Ok(())
    }
}
