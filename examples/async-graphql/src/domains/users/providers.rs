use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::inject;
use std::sync::Arc;

use super::service::{UserLoader, UsersService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(UsersService)
pub const USERS_SERVICE: inject::Tag<Arc<UsersService>> = inject::Tag::new("UsersService");

/// Provide the UsersService
///
/// **Provides:** `Arc<UsersService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideUsersService {}

#[async_trait]
impl inject::Provider<Arc<UsersService>> for ProvideUsersService {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<UsersService>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(UsersService::new(db.clone())))
    }
}

/// Tag(UserLoader)
pub const USER_LOADER: inject::Tag<DataLoader<UserLoader>> = inject::Tag::new("UserLoader");

/// Provide the UserLoader
///
/// **Provides:** `UserLoader`
///
/// **Depends on:**
///  - `Tag(UsersService)`
#[derive(Default)]
pub struct ProvideUserLoader {}

#[async_trait]
impl inject::Provider<DataLoader<UserLoader>> for ProvideUserLoader {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<DataLoader<UserLoader>> {
        let users_service = i.get(&USERS_SERVICE)?;

        Ok(DataLoader::new(
            UserLoader::new(users_service.clone()),
            tokio::spawn,
        ))
    }
}
