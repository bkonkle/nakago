use async_graphql::{EmptySubscription, Schema};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provider, Tag};
use std::sync::Arc;

use crate::domains::shows::{
    resolver::{ShowsMutation, ShowsQuery},
    service::SHOWS_SERVICE,
};

/// Tag(ShowsSchema)
#[allow(dead_code)]
pub const SHOWS_SCHEMA: Tag<Box<ShowsSchema>> = Tag::new("ShowsSchema");

/// The ShowsSchema, covering just the Shows domain
pub type ShowsSchema = Schema<ShowsQuery, ShowsMutation, EmptySubscription>;

/// Provide the ShowsSchema
///
/// **Provides:** `Arc<ShowsSchema>`
///
/// **Depends on:**
///   - `Tag(ShowsService)`
///   - `Tag(ShowLoader)`
#[derive(Default)]
pub struct ProvideShowsSchema {}

#[async_trait]
impl Provider<ShowsSchema> for ProvideShowsSchema {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<ShowsSchema>> {
        let service = i.get(&SHOWS_SERVICE).await?;

        let schema: ShowsSchema = Schema::build(
            ShowsQuery::default(),
            ShowsMutation::default(),
            EmptySubscription,
        )
        .data(service)
        .finish();

        Ok(Arc::new(schema))
    }
}
