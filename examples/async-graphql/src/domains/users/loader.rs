use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provide, Tag};

use super::{model::User, service::Service, USERS_SERVICE};

/// Tag(UserLoader)
pub const USER_LOADER: Tag<DataLoader<UserLoader>> = Tag::new("UserLoader");

/// Provide the UserLoader
///
/// **Provides:** `UserLoader`
///
/// **Depends on:**
///  - `Tag(UsersService)`
#[derive(Default)]
pub struct Provider {}

#[async_trait]
impl Provide<DataLoader<UserLoader>> for Provider {
    async fn provide(&self, i: &Inject) -> InjectResult<DataLoader<UserLoader>> {
        let users_service = i.get(&USERS_SERVICE)?;

        Ok(DataLoader::new(
            UserLoader::new(users_service.clone()),
            tokio::spawn,
        ))
    }
}

/// A dataloader for `User` instances
pub struct UserLoader {
    /// The SeaOrm database connection
    locations: Arc<dyn Service>,
}

/// The default implementation for the `UserLoader`
impl UserLoader {
    /// Create a new instance
    pub fn new(locations: Arc<dyn Service>) -> Self {
        Self { locations }
    }
}

#[async_trait]
impl Loader<String> for UserLoader {
    type Value = User;
    type Error = FieldError;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        let locations = self.locations.get_by_ids(keys.into()).await?;

        Ok(locations
            .into_iter()
            .map(|location| (location.id.clone(), location))
            .collect())
    }
}
