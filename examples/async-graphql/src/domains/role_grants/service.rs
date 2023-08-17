use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;
use nakago_sea_orm::{DatabaseConnection, DATABASE_CONNECTION};
use sea_orm::{entity::*, query::*, Condition, EntityTrait};

use super::model::{self, CreateRoleGrantInput, RoleGrant};

/// Tag(RoleGrantsService)
pub const ROLE_GRANTS_SERVICE: Tag<Box<dyn RoleGrantsService>> = Tag::new("RoleGrantsService");

/// A RoleGrantsService appliies business logic to a dynamic RoleGrantsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait RoleGrantsService: Sync + Send {
    /// Get an individual `RoleGrant` by id
    async fn get(&self, id: &str) -> Result<Option<RoleGrant>>;

    /// Get a list of `RoleGrant` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<RoleGrant>>;

    /// Create a `RoleGrant` with the given input
    async fn create(&self, input: &CreateRoleGrantInput) -> Result<RoleGrant>;

    /// Delete an existing `RoleGrant`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `RoleGrantsService` struct.
pub struct DefaultRoleGrantsService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `RoleGrantsService` implementation
impl DefaultRoleGrantsService {
    /// Create a new `RoleGrantsService` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoleGrantsService for DefaultRoleGrantsService {
    async fn get(&self, id: &str) -> Result<Option<RoleGrant>> {
        let query = model::Entity::find_by_id(id.to_owned());

        let role_grant = query.one(&*self.db).await?;

        Ok(role_grant)
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<RoleGrant>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let role_grants = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(role_grants)
    }

    async fn create(&self, input: &CreateRoleGrantInput) -> Result<RoleGrant> {
        let role_grant = model::ActiveModel {
            role_key: Set(input.role_key.clone()),
            user_id: Set(input.user_id.clone()),
            resource_table: Set(input.resource_table.clone()),
            resource_id: Set(input.resource_id.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let created: RoleGrant = role_grant;

        return Ok(created);
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let role_grant = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find RoleGrant with id: {}", id))?;

        let _result = role_grant.delete(&*self.db).await?;

        Ok(())
    }
}

/// Provide the RoleGrantsService
///
/// **Provides:** `Arc<dyn RoleGrantsService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideRoleGrantsService {}

#[Provider]
#[async_trait]
impl Provider<Box<dyn RoleGrantsService>> for ProvideRoleGrantsService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Box<dyn RoleGrantsService>>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        Ok(Arc::new(Box::new(DefaultRoleGrantsService::new(db))))
    }
}
