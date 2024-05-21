use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
#[cfg(test)]
use mockall::automock;
use nakago::{provider, Inject, Provider, Tag};
use nakago_derive::Provider;
use nakago_sea_orm::{DatabaseConnection, CONNECTION};
use sea_orm::{entity::*, query::*, Condition, EntityTrait};

use super::model::{self, CreateRoleGrantInput, RoleGrant};

/// Tag(role_grants::Service)
pub const SERVICE: Tag<Box<dyn Service>> = Tag::new("role_grants::Service");

/// A Service appliies business logic to a dynamic RoleGrantsRepository implementation.
#[cfg_attr(test, automock)]
#[allow(unused)]
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
#[derive(new)]
pub struct DefaultService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
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

/// Provide the Service
///
/// **Provides:** `Arc<Box<dyn role_grants::Service>>`
///
/// **Depends on:**
///   - `nakago_sea_orm::DatabaseConnection`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Box<dyn Service>> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Box<dyn Service>>> {
        let db = i.get(&CONNECTION).await?;

        Ok(Arc::new(Box::new(DefaultService::new(db))))
    }
}
