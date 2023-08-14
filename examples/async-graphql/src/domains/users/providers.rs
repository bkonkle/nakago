use async_graphql::dataloader::DataLoader;
use async_trait::async_trait;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};
use std::sync::Arc;

use super::service::{DefaultUsersService, UserLoader, UsersService};
use crate::db::providers::DATABASE_CONNECTION;

/// Tag(UsersService)
pub const USERS_SERVICE: Tag<Box<dyn UsersService>> = Tag::new("UsersService");

/// Provide the UsersService
///
/// **Provides:** `Arc<dyn UsersServiceTrait>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideUsersService {}

#[async_trait]
impl Provider for ProvideUsersService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        let service: Box<dyn UsersService> = Box::new(DefaultUsersService::new(db.clone()));

        Ok(Arc::new(service))
    }
}

/// Tag(UserLoader)
pub const USER_LOADER: Tag<DataLoader<UserLoader>> = Tag::new("UserLoader");

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
