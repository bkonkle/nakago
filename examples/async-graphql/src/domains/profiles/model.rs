#![allow(missing_docs)]

use async_graphql::SimpleObject;
use chrono::Utc;
use fake::Dummy;
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domains::users::model::{self as user_model, User};

/// The `Profile` GraphQL model
#[derive(Debug, Dummy, Clone, Eq, PartialEq, Deserialize, PolarClass, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct Profile {
    /// The `Profile` id
    #[polar(attribute)]
    pub id: String,

    /// The date the `Profile` was created
    pub created_at: DateTime,

    /// The date the `Profile` was last updated
    pub updated_at: DateTime,

    /// The `Profile`'s email address
    // (differs from DB)
    // Optional because this field may be censored for unauthorized users
    #[polar(attribute)]
    pub email: Option<String>,

    /// The `Profile`'s display name
    pub display_name: Option<String>,

    /// The `Profile`'s picture
    pub picture: Option<String>,

    /// The `Profile`'s city
    pub city: Option<String>,

    /// The `Profile`'s state or province
    pub state_province: Option<String>,

    /// The `Profile`'s `User` id
    #[polar(attribute)]
    pub user_id: Option<String>,

    /// The associated `User`
    #[graphql(skip)]
    pub user: Option<User>,
}

impl Profile {
    /// If not authorized, censor the `Profile` `email` and `user_id`
    pub fn censor(&self, current_user_id: &Option<String>) -> Self {
        let mut profile = self.clone();

        let same_user = match current_user_id {
            Some(user_id) => self.user_id == Some(user_id.clone()),
            _ => false,
        };

        // If not same user, censor the record
        if same_user {
            profile.email.clone_from(&self.email);
            profile.user_id.clone_from(&self.user_id);
            profile.user.clone_from(&self.user);
        } else {
            profile.email = None;
            profile.user_id = None;
            profile.user = None;
        }

        profile
    }
}

/// The `Profile` Database model
#[derive(Clone, Debug, Dummy, Eq, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "profiles")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Text")]
    pub id: String,

    pub created_at: DateTime,

    pub updated_at: DateTime,

    #[sea_orm(column_type = "Text")]
    pub email: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub display_name: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub picture: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub city: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub state_province: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub user_id: Option<String>,
}

impl Model {
    pub fn into_profile_with_user(self, user: User) -> Profile {
        Profile {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            email: Some(self.email),
            display_name: self.display_name,
            picture: self.picture,
            city: self.city,
            state_province: self.state_province,
            user_id: Some(user.id.clone()),
            user: Some(user),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            email: String::default(),
            display_name: Option::default(),
            picture: Option::default(),
            city: Option::default(),
            state_province: Option::default(),
            user_id: Option::default(),
        }
    }
}

/// `Profile` entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_model::Entity",
        from = "Column::UserId",
        to = "user_model::Column::Id"
    )]
    User,
}

impl Related<user_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Profile {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            email: Some(model.email),
            display_name: model.display_name,
            picture: model.picture,
            city: model.city,
            state_province: model.state_province,
            user_id: model.user_id,
            user: None,
        }
    }
}

/// A wrapper around a `Vec<Profile` to enable trait implementations
pub struct ProfileList(Vec<Profile>);

impl ProfileList {
    /// Proxy to the `Vec` `len` method
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Proxy to the `Vec` `is_empty` method
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<Model>> for ProfileList {
    fn from(data: Vec<Model>) -> ProfileList {
        ProfileList(data.into_iter().map(|p| p.into()).collect())
    }
}

impl From<Vec<(Model, Option<User>)>> for ProfileList {
    fn from(data: Vec<(Model, Option<User>)>) -> ProfileList {
        ProfileList(
            data.into_iter()
                .map(|(profile, user)| Profile {
                    user,
                    ..profile.into()
                })
                .collect(),
        )
    }
}

impl From<ProfileList> for Vec<Profile> {
    fn from(profiles: ProfileList) -> Vec<Profile> {
        profiles.0
    }
}

/// A wrapper around `Option<Profile>` to enable trait implementations
pub struct ProfileOption(pub Option<Profile>);

impl From<Option<Model>> for ProfileOption {
    fn from(data: Option<Model>) -> ProfileOption {
        ProfileOption(data.map(|p| p.into()))
    }
}

impl From<Option<(Model, Option<User>)>> for ProfileOption {
    fn from(data: Option<(Model, Option<User>)>) -> ProfileOption {
        ProfileOption(data.map(|(profile, user)| Profile {
            user,
            ..profile.into()
        }))
    }
}

impl From<ProfileOption> for Option<Profile> {
    fn from(profile: ProfileOption) -> Option<Profile> {
        profile.0
    }
}
