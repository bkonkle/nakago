use std::any::Any;

use async_graphql::{ObjectType, Schema, SchemaBuilder, SubscriptionType};
use derive_new::new;
use nakago::{Inject, Tag};

/// Initialize the GraphQL Schema and inject it with the given Tag
#[derive(new)]
pub struct Init<Query: Any, Mutation: Any, Subscription: Any> {
    builder_tag: Option<&'static Tag<SchemaBuilder<Query, Mutation, Subscription>>>,
    schema_tag: Option<&'static Tag<Schema<Query, Mutation, Subscription>>>,
}

// Implement manually rather than deriving, to avoid error messages for consumers like "the trait
// `std::default::Default` is not implemented for `graphql::Query`"
impl<Query: Any, Mutation: Any, Subscription: Any> Default for Init<Query, Mutation, Subscription> {
    fn default() -> Self {
        Self {
            builder_tag: None,
            schema_tag: None,
        }
    }
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

    /// Initialize the schema and inject it
    pub async fn init(&self, inject: &Inject) -> nakago::Result<()>
    where
        Query: ObjectType + 'static,
        Mutation: ObjectType + 'static,
        Subscription: SubscriptionType + 'static,
    {
        let schema_builder = if let Some(tag) = self.builder_tag {
            inject.consume_tag(tag).await?
        } else {
            inject
                .consume::<SchemaBuilder<Query, Mutation, Subscription>>()
                .await?
        };

        let schema = schema_builder.finish();

        if let Some(tag) = self.schema_tag {
            inject.inject_tag(tag, schema).await?;
        } else {
            inject.inject(schema).await?;
        }

        Ok(())
    }
}
