use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::inject;

use super::service::{DefaultProfilesService, ProfileLoader, ProfilesService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(ProfilesService)
pub const PROFILES_SERVICE: inject::Tag<Arc<dyn ProfilesService>> =
    inject::Tag::new("ProfilesService");

/// Provide the ProfilesService
///
/// **Provides:** `Arc<dyn ProfilesService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideProfilesService {}

#[async_trait]
impl inject::Provider<Arc<dyn ProfilesService>> for ProvideProfilesService {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<dyn ProfilesService>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(DefaultProfilesService::new(db.clone())))
    }
}

/// Tag(ProfileLoader)
pub const PROFILE_LOADER: inject::Tag<Arc<DataLoader<ProfileLoader>>> =
    inject::Tag::new("ProfileLoader");

/// Provide the ProfileLoader
///
/// **Provides:** `ProfileLoader`
///
/// **Depends on:**
///  - `Tag(ProfilesService)`
#[derive(Default)]
pub struct ProvideProfileLoader {}

#[async_trait]
impl inject::Provider<Arc<DataLoader<ProfileLoader>>> for ProvideProfileLoader {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<DataLoader<ProfileLoader>>> {
        let profiles_service = i.get(&PROFILES_SERVICE)?;

        Ok(Arc::new(DataLoader::new(
            ProfileLoader::new(profiles_service.clone()),
            tokio::spawn,
        )))
    }
}
