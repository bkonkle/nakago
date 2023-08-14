use std::{collections::HashMap, sync::Arc};

use async_graphql::{
    dataloader::{DataLoader, Loader},
    FieldError,
};
use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};

use super::{
    model::User,
    service::{UsersService, USERS_SERVICE},
};

/// Tag(UserLoader)
pub const USER_LOADER: Tag<DataLoader<UserLoader>> = Tag::new("UserLoader");

/// A dataloader for `User` instances
pub struct UserLoader {
    /// The SeaOrm database connection
    locations: Arc<Box<dyn UsersService>>,
}

/// The default implementation for the `UserLoader`
impl UserLoader {
    /// Create a new instance
    pub fn new(locations: Arc<Box<dyn UsersService>>) -> Self {
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

/// Provide the UserLoader
///
/// **Provides:** `UserLoader`
///
/// **Depends on:**
///  - `Tag(UsersService)`
#[derive(Default)]
pub struct ProvideUserLoader {}

#[async_trait]
impl Provider for ProvideUserLoader {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let users_service = i.get(&USERS_SERVICE).await?;

        Ok(Arc::new(DataLoader::new(
            UserLoader::new(users_service.clone()),
            tokio::spawn,
        )))
    }
}
