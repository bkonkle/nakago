use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::inject;
use std::sync::Arc;

use super::service::{DefaultShowsService, ShowLoader, ShowsService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(ShowsService)
pub const SHOWS_SERVICE: inject::Tag<Arc<dyn ShowsService>> = inject::Tag::new("ShowsService");

/// Provide the ShowsService
///
/// **Provides:** `Arc<dyn ShowsService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideShowsService {}

#[async_trait]
impl inject::Provider<Arc<dyn ShowsService>> for ProvideShowsService {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<dyn ShowsService>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(DefaultShowsService::new(db.clone())))
    }
}

/// Tag(ShowLoader)
pub const SHOW_LOADER: inject::Tag<Arc<DataLoader<ShowLoader>>> = inject::Tag::new("ShowLoader");

/// Provide the ShowLoader
///
/// **Provides:** `ShowLoader`
///
/// **Depends on:**
///  - `Tag(ShowsService)`
#[derive(Default)]
pub struct ProvideShowLoader {}

#[async_trait]
impl inject::Provider<Arc<DataLoader<ShowLoader>>> for ProvideShowLoader {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<DataLoader<ShowLoader>>> {
        let shows_service = i.get(&SHOWS_SERVICE)?;

        Ok(Arc::new(DataLoader::new(
            ShowLoader::new(shows_service.clone()),
            tokio::spawn,
        )))
    }
}
