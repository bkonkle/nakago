use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Tag};

// Data is provided to support GraphQL resolver operations
type Data = dyn Any + Send + Sync;

/// Tag(AsyncGraphQL:Data)
pub const GRAPHQL_DATA: Tag<Vec<Arc<Data>>> = Tag::new("AsyncGraphQL:Data");

/// Add the given GraphQL supporting data to the stack.
///
/// **Provides or Modifies:**
///   - `Tag(AsyncGraphQL:Data)`
pub struct AddGraphQLData {
    data: Vec<Arc<Data>>,
}

impl AddGraphQLData {
    /// Create a new AddGraphQLDatas instance
    pub fn new(data: Vec<Arc<Data>>) -> Self {
        Self { data }
    }
}

#[async_trait]
impl Hook for AddGraphQLData {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let data = match i.consume(&GRAPHQL_DATA).await {
            Ok(data) => {
                let mut updated = data.clone();

                // Add the given data to the stack
                for loader in self.data.iter() {
                    updated.push(loader.clone());
                }

                updated
            }
            Err(_) => self.data.clone(),
        };

        let _ = i.override_tag(&GRAPHQL_DATA, data).await?;

        Ok(())
    }
}
