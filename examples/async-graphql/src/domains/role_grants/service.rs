use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use nakago::{Inject, InjectResult, Provide, Tag};
use sea_orm::{entity::*, query::*, Condition, DatabaseConnection, EntityTrait};

use crate::db::provider::DATABASE_CONNECTION;

use super::model::{self, CreateRoleGrantInput, RoleGrant};

/// Tag(RoleGrantsService)
pub const ROLE_GRANTS_SERVICE: Tag<Arc<dyn Service>> = Tag::new("RoleGrantsService");

/// Provide the RoleGrantsService
///
/// **Provides:** `Arc<dyn Service>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct Provider {}

#[async_trait]
impl Provide<Arc<dyn Service>> for Provider {
    async fn provide(&self, i: &Inject) -> InjectResult<Arc<dyn Service>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(DefaultService::new(db.clone())))
    }
}

/// A Service appliies business logic to a dynamic RoleGrantsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Service: Sync + Send {
    /// Get an individual `RoleGrant` by id
    async fn get(&self, id: &str) -> Result<Option<RoleGrant>>;

    /// Get a list of `RoleGrant` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<RoleGrant>>;

    /// Create a `RoleGrant` with the given input
    async fn create(&self, input: &CreateRoleGrantInput) -> Result<RoleGrant>;

    /// Delete an existing `RoleGrant`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `Service` struct.
pub struct DefaultService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `Service` implementation
impl DefaultService {
    /// Create a new `Service` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Service for DefaultService {
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
