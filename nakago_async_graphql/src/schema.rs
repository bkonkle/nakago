use std::{any::Any, marker::PhantomData, sync::Arc};

use async_graphql::{ObjectType, Schema, SchemaBuilder, SubscriptionType};
use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;

/// Provides a SchemaBuilder that can be modified by hooks to initialize the GraphQL Schema
///
/// **Provides:** `SchemaBuilder<Query, Mutation, Subscription>`
#[derive(Default)]
pub struct SchemaBuilderProvider<Query: Any, Mutation: Any, Subscription: Any> {
    _query: PhantomData<Query>,
    _mutation: PhantomData<Mutation>,
    _subscription: PhantomData<Subscription>,
}

#[Provider]
#[async_trait]
impl<Query, Mutation, Subscription> Provider<SchemaBuilder<Query, Mutation, Subscription>>
    for SchemaBuilderProvider<Query, Mutation, Subscription>
where
    Query: ObjectType + Default + 'static,
    Mutation: ObjectType + Default + 'static,
    Subscription: SubscriptionType + Default + 'static,
{
    async fn provide(
        self: Arc<Self>,
        _i: Inject,
    ) -> InjectResult<Arc<SchemaBuilder<Query, Mutation, Subscription>>> {
        Ok(Arc::new(Schema::build(
            Query::default(),
            Mutation::default(),
            Subscription::default(),
        )))
    }
}

/// Initialize the GraphQL Schema and inject it with the given Tag
#[derive(Default)]
pub struct InitSchema<Query: Any, Mutation: Any, Subscription: Any> {
    builder: Option<&'static Tag<SchemaBuilder<Query, Mutation, Subscription>>>,
    schema: Option<&'static Tag<Schema<Query, Mutation, Subscription>>>,
}

impl<Query: Any, Mutation: Any, Subscription: Any> InitSchema<Query, Mutation, Subscription> {
    /// Add a builder tag to the hook
    pub fn with_builder_tag(
        self,
        builder: &'static Tag<SchemaBuilder<Query, Mutation, Subscription>>,
    ) -> Self {
        Self {
            builder: Some(builder),
            ..self
        }
    }

    /// Add a schema tag to the hook
    pub fn with_schema_tag(
        self,
        schema: &'static Tag<Schema<Query, Mutation, Subscription>>,
    ) -> Self {
        Self {
            schema: Some(schema),
            ..self
        }
    }
}

#[async_trait]
impl<Query, Mutation, Subscription> Hook for InitSchema<Query, Mutation, Subscription>
where
    Query: ObjectType + Default + 'static,
    Mutation: ObjectType + Default + 'static,
    Subscription: SubscriptionType + Default + 'static,
{
    async fn handle(&self, inject: Inject) -> InjectResult<()> {
        let schema_builder = if let Some(tag) = self.builder {
            inject.consume(tag).await?
        } else {
            inject
                .consume_type::<SchemaBuilder<Query, Mutation, Subscription>>()
                .await?
        };

        let schema = schema_builder.finish();

        if let Some(tag) = self.schema {
            inject.inject(tag, schema).await?;
        } else {
            inject.inject_type(schema).await?;
        }

        Ok(())
    }
}
