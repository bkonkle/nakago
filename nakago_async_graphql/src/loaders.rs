use std::{any::Any, hash::Hash, sync::Arc};

use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Tag};

type AnyLoader =
    dyn Loader<dyn Any + Send + Sync, Value = dyn Any + Send + Sync, Error = dyn Any + Send>;

/// Tag(AsyncGraphQL:DataLoaders)
pub const GRAPHQL_DATA_LOADERS: Tag<Vec<Arc<DataLoader<AnyLoader>>>> =
    Tag::new("AsyncGraphQL:DataLoaders");

/// Add the given GraphQL supporting service to the stack.
///
/// **Provides or Modifies:**
///   - `Tag(AsyncGraphQL:DataLoaders)`
pub struct AddGraphQLDataLoaders {
    loaders: Vec<Arc<DataLoader<AnyLoader>>>,
}

impl AddGraphQLDataLoaders {
    /// Create a new AddGraphQLDataLoaders instance
    pub fn new(loaders: Vec<Arc<DataLoader<AnyLoader>>>) -> Self {
        Self { loaders }
    }
}

#[async_trait]
impl Hook for AddGraphQLDataLoaders {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let loaders = match i.consume(&GRAPHQL_DATA_LOADERS).await {
            Ok(loaders) => {
                let mut updated = loaders.clone();

                // Add the given loaders to the stack
                for loader in self.loaders.iter() {
                    updated.push(loader.clone());
                }

                updated
            }
            Err(_) => self.loaders.clone(),
        };

        let _ = i.override_tag(&GRAPHQL_DATA_LOADERS, loaders).await?;

        Ok(())
    }
}
