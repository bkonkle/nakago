use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provider, Tag};

use super::{
    model::Profile,
    service::{ProfilesService, PROFILES_SERVICE},
};

/// Tag(ProfileLoader)
pub const PROFILE_LOADER: Tag<DataLoader<ProfileLoader>> = Tag::new("ProfileLoader");

/// A dataloader for `Profile` instances
pub struct ProfileLoader {
    /// The SeaOrm database connection
    profiles: Arc<Box<dyn ProfilesService>>,
}

/// The default implementation for the `ProfileLoader`
impl ProfileLoader {
    /// Create a new instance
    pub fn new(profiles: Arc<Box<dyn ProfilesService>>) -> Self {
        Self { profiles }
    }
}

#[async_trait]
impl Loader<String> for ProfileLoader {
    type Value = Profile;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let profiles = self.profiles.get_by_ids(keys.into()).await?;

        Ok(profiles
            .into_iter()
            .map(|profile| (profile.id.clone(), profile))
            .collect())
    }
}

/// Provide the ProfileLoader
///
/// **Provides:** `ProfileLoader`
///
/// **Depends on:**
///  - `Tag(ProfilesService)`
#[derive(Default)]
pub struct ProvideProfileLoader {}

#[async_trait]
impl Provider<DataLoader<ProfileLoader>> for ProvideProfileLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<DataLoader<ProfileLoader>>> {
        let profiles_service = i.get(&PROFILES_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            ProfileLoader::new(profiles_service.clone()),
            tokio::spawn,
        )))
    }
}
