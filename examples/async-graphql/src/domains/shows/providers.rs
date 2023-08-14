use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};
use std::sync::Arc;

use super::service::{DefaultShowsService, ShowLoader, ShowsService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(ShowsService)
pub const SHOWS_SERVICE: Tag<Box<dyn ShowsService>> = Tag::new("ShowsService");

/// Provide the ShowsService
///
/// **Provides:** `Arc<dyn ShowsService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideShowsService {}

#[async_trait]
impl Provider for ProvideShowsService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        let service: Box<dyn ShowsService> = Box::new(DefaultShowsService::new(db));

        Ok(Arc::new(service))
    }
}

/// Tag(ShowLoader)
pub const SHOW_LOADER: Tag<DataLoader<ShowLoader>> = Tag::new("ShowLoader");

/// Provide the ShowLoader
///
/// **Provides:** `ShowLoader`
///
/// **Depends on:**
///  - `Tag(ShowsService)`
#[derive(Default)]
pub struct ProvideShowLoader {}

#[async_trait]
impl Provider for ProvideShowLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let shows_service = i.get(&SHOWS_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            ShowLoader::new(shows_service.clone()),
            tokio::spawn,
        )))
    }
}
