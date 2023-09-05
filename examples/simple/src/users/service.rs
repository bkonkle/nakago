use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;
use nakago_sea_orm::{DatabaseConnection, DATABASE_CONNECTION};
use sea_orm::{entity::*, query::*, EntityTrait};

use super::model::{self, User, UserOption};

/// Tag(UsersService)
pub const USERS_SERVICE: Tag<Box<dyn UsersService>> = Tag::new("UsersService");

/// A UsersService appliies business logic to a dynamic UsersRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UsersService: Sync + Send {
    /// Get an individual `User` by id
    async fn get(&self, id: &str) -> Result<Option<User>>;

    /// Get a list of `User` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<User>>;

    /// Get an individual `User` by username
    async fn get_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Create a `User` with the given username
    async fn create(&self, username: &str) -> Result<User>;

    /// Get the `User` with the given username, creating one if none are found
    async fn get_or_create(&self, username: &str) -> Result<User>;

    /// Update an existing `User`
    async fn update(&self, id: &str, input: &UpdateUserInput) -> Result<User>;

    /// Delete an existing `User`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The `UpdateUserInput` input type
#[derive(Clone, Default, Eq, PartialEq)]
pub struct UpdateUserInput {
    /// The User's subscriber id
    pub username: Option<String>,

    /// The User's display name
    pub display_name: Option<String>,
}

/// The default `UsersService` implementation
pub struct DefaultUsersService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `UsersService` implementation
impl DefaultUsersService {
    /// Create a new `DefaultUsersService` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UsersService for DefaultUsersService {
    async fn get(&self, id: &str) -> Result<Option<User>> {
        let user = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?;

        Ok(user)
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<User>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let users = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(users)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        let query = model::Entity::find().filter(model::Column::Username.eq(username.to_owned()));

        let user: UserOption = query.one(&*self.db).await?.into();

        Ok(user.into())
    }

    async fn create(&self, username: &str) -> Result<User> {
        let user = model::ActiveModel {
            username: Set(username.to_owned()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        Ok(user)
    }

    async fn get_or_create(&self, username: &str) -> Result<User> {
        match self.get_by_username(username).await? {
            Some(user) => Ok(user),
            None => self.create(username).await,
        }
    }

    async fn update(&self, id: &str, input: &UpdateUserInput) -> Result<User> {
        let query = model::Entity::find_by_id(id.to_owned());

        let user = query
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find User with id: {}", id))?;

        let mut user: model::ActiveModel = user.into();

        if let Some(username) = &input.username {
            user.username = Set(username.clone());
        }

        if let Some(display_name) = &input.display_name {
            user.display_name = Set(display_name.clone());
        }

        let updated = user.update(&*self.db).await?;

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let user = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find User with id: {}", id))?;

        let _result = user.delete(&*self.db).await?;

        Ok(())
    }
}

/// Provide the UsersService
///
/// **Provides:** `Arc<dyn UsersServiceTrait>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideUsersService {}

#[Provider]
#[async_trait]
impl Provider<Box<dyn UsersService>> for ProvideUsersService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Box<dyn UsersService>>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        Ok(Arc::new(Box::new(DefaultUsersService::new(db))))
    }
}
