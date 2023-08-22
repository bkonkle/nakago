use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult, Tag};

// A Service supports GraphQL resolver operations
// TODO: Move this to a trait once middleware for Services is supported
type Service = dyn Any + Send + Sync;

/// Tag(AsyncGraphQL:Services)
pub const GRAPHQL_SERVICES: Tag<Vec<Arc<Service>>> = Tag::new("AsyncGraphQL:Services");

/// Add the given GraphQL supporting service to the stack.
///
/// **Provides or Modifies:**
///   - `Tag(AsyncGraphQL:Services)`
pub struct AddGraphQLServices {
    services: Vec<Arc<Service>>,
}

impl AddGraphQLServices {
    /// Create a new AddGraphQLServices instance
    pub fn new(services: Vec<Arc<Service>>) -> Self {
        Self { services }
    }
}

#[async_trait]
impl Hook for AddGraphQLServices {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let services = match i.consume(&GRAPHQL_SERVICES).await {
            Ok(services) => {
                let mut updated = services.clone();

                // Add the given services to the stack
                for loader in self.services.iter() {
                    updated.push(loader.clone());
                }

                updated
            }
            Err(_) => self.services.clone(),
        };

        let _ = i.override_tag(&GRAPHQL_SERVICES, services).await?;

        Ok(())
    }
}
