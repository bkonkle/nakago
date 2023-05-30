use anyhow::Result;
use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provide, Tag};
use std::{collections::HashMap, sync::Arc};

use super::{
    model::Profile,
    service::{Service, PROFILES_SERVICE},
};

/// Tag(ProfileLoader)
pub const PROFILE_LOADER: Tag<DataLoader<ProfileLoader>> = Tag::new("ProfileLoader");

/// Provide the ProfileLoader
///
/// **Provides:** `ProfileLoader`
///
/// **Depends on:**
///  - `Tag(ProfilesService)`
#[derive(Default)]
pub struct Provider {}

#[async_trait]
impl Provide<DataLoader<ProfileLoader>> for Provider {
    async fn provide(&self, i: &Inject) -> InjectResult<DataLoader<ProfileLoader>> {
        let profiles_service = i.get(&PROFILES_SERVICE)?;

        Ok(DataLoader::new(
            ProfileLoader::new(profiles_service.clone()),
            tokio::spawn,
        ))
    }
}

/// A dataloader for `Profile` instances
pub struct ProfileLoader {
    /// The SeaOrm database connection
    profiles: Arc<dyn Service>,
}

/// The default implementation for the `ProfileLoader`
impl ProfileLoader {
    /// Create a new instance
    pub fn new(profiles: Arc<dyn Service>) -> Self {
        Self {
            profiles: profiles.clone(),
        }
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
