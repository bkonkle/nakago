#![allow(missing_docs)]

use chrono::Utc;
use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// The User GraphQL and Database Model
#[derive(Clone, Debug, Dummy, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// The User id
    #[sea_orm(primary_key, column_type = "Text")]
    pub id: String,

    /// The date the User was created
    pub created_at: DateTime,

    /// The date the User was last updated
    pub updated_at: DateTime,

    /// The User's subscriber id
    #[sea_orm(column_type = "Text")]
    pub username: String,

    /// The User's display name
    #[sea_orm(column_type = "Text")]
    pub display_name: String,
}

/// The User GraphQL type is the same as the database Model
pub type User = Model;

/// User entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            username: String::default(),
            display_name: String::default(),
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

impl From<UserOption> for Option<User> {
    fn from(user: UserOption) -> Option<User> {
        user.0
    }
}
