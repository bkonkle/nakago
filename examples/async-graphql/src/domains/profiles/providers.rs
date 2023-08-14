use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};

use super::service::{DefaultProfilesService, ProfileLoader, ProfilesService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(ProfilesService)
pub const PROFILES_SERVICE: Tag<Box<dyn ProfilesService>> = Tag::new("ProfilesService");

/// Provide the ProfilesService
///
/// **Provides:** `Arc<dyn ProfilesService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideProfilesService {}

#[async_trait]
impl Provider for ProvideProfilesService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        let service: Box<dyn ProfilesService> = Box::new(DefaultProfilesService::new(db.clone()));

        Ok(Arc::new(service))
    }
}

/// Tag(ProfileLoader)
pub const PROFILE_LOADER: Tag<DataLoader<ProfileLoader>> = Tag::new("ProfileLoader");

/// Provide the ProfileLoader
///
/// **Provides:** `ProfileLoader`
///
/// **Depends on:**
///  - `Tag(ProfilesService)`
#[derive(Default)]
pub struct ProvideProfileLoader {}

#[async_trait]
impl Provider for ProvideProfileLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let profiles_service = i.get(&PROFILES_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            ProfileLoader::new(profiles_service.clone()),
            tokio::spawn,
        )))
    }
}
