#![allow(missing_docs)]

use async_graphql::SimpleObject;
use chrono::Utc;
use fake::Dummy;
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domains::role_grants::model::{self as role_grant_model, RoleGrant};

/// The User GraphQL and Database Model
#[derive(
    Clone,
    Debug,
    Dummy,
    Eq,
    PartialEq,
    DeriveEntityModel,
    Deserialize,
    Serialize,
    SimpleObject,
    PolarClass,
)]
#[graphql(name = "User")]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// The User id
    #[sea_orm(primary_key, column_type = "Text")]
    #[polar(attribute)]
    pub id: String,

    /// The date the User was created
    pub created_at: DateTime,

    /// The date the User was last updated
    pub updated_at: DateTime,

    /// The User's subscriber id
    #[sea_orm(column_type = "Text")]
    #[polar(attribute)]
    pub username: String,

    /// Whether the User is active or disabled
    #[polar(attribute)]
    pub is_active: bool,

    /// Related RoleGrants
    #[sea_orm(ignore)]
    #[polar(attribute)]
    pub roles: Vec<RoleGrant>,
}

/// The User GraphQL type is the same as the database Model
pub type User = Model;

/// User entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "role_grant_model::Entity")]
    RoleGrant,
}

impl Related<role_grant_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleGrant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            username: String::default(),
            is_active: bool::default(),
            roles: Vec::default(),
        }
    }
}

/// A wrapper around `Option<User>` to enable the trait implementations below
pub struct UserOption(pub Option<User>);

impl From<Option<Model>> for UserOption {
    fn from(data: Option<Model>) -> UserOption {
        UserOption(data)
    }
}

impl From<Option<(Model, Vec<RoleGrant>)>> for UserOption {
    fn from(data: Option<(Model, Vec<RoleGrant>)>) -> UserOption {
        UserOption(data.map(|(user, roles)| User { roles, ..user }))
    }
}

impl From<UserOption> for Option<User> {
    fn from(user: UserOption) -> Option<User> {
        user.0
    }
}
