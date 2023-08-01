use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::inject;
use std::sync::Arc;

use super::service::{DefaultUsersService, UserLoader, UsersService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(UsersService)
pub const USERS_SERVICE: inject::Tag<Arc<dyn UsersService>> = inject::Tag::new("UsersService");

/// Provide the UsersService
///
/// **Provides:** `Arc<dyn UsersServiceTrait>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideUsersService {}

#[async_trait]
impl inject::Provider<Arc<dyn UsersService>> for ProvideUsersService {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<dyn UsersService>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(DefaultUsersService::new(db.clone())))
    }
}

/// Tag(UserLoader)
pub const USER_LOADER: inject::Tag<Arc<DataLoader<UserLoader>>> = inject::Tag::new("UserLoader");

/// Provide the UserLoader
///
/// **Provides:** `UserLoader`
///
/// **Depends on:**
///  - `Tag(UsersService)`
#[derive(Default)]
pub struct ProvideUserLoader {}

#[async_trait]
impl inject::Provider<Arc<DataLoader<UserLoader>>> for ProvideUserLoader {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<DataLoader<UserLoader>>> {
        let users_service = i.get(&USERS_SERVICE)?;

        Ok(Arc::new(DataLoader::new(
            UserLoader::new(users_service.clone()),
            tokio::spawn,
        )))
    }
}
