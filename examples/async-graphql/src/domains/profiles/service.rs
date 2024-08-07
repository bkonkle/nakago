use std::sync::Arc;

use anyhow::Result;
use async_graphql::MaybeUndefined::{Null, Undefined, Value};
use async_trait::async_trait;
use derive_new::new;
#[cfg(test)]
use mockall::automock;
use nakago::{provider, Inject, Provider};
use nakago_axum::utils::{ManyResponse, Ordering};
use nakago_derive::Provider;
use nakago_sea_orm::DatabaseConnection;
use sea_orm::{entity::*, query::*, EntityTrait};

use crate::domains::users::model as user_model;

use super::{
    model::{self, Profile, ProfileList, ProfileOption},
    mutation::{CreateProfileInput, UpdateProfileInput},
    query::{ProfileCondition, ProfilesOrderBy},
};

/// A Service applies business logic to a dynamic ProfilesRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Service: Sync + Send {
    /// Get an individual `Profile` by id
    async fn get(&self, id: &str, with_user: &bool) -> Result<Option<Profile>>;

    /// Get a list of `Profile` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Profile>>;

    /// Get multiple `Profile` records
    async fn get_many(
        &self,
        condition: Option<ProfileCondition>,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page_size: Option<u64>,
        page: Option<u64>,
        with_user: &bool,
    ) -> Result<ManyResponse<Profile>>;

    /// Get the first `Profile` with this user_id
    async fn get_by_user_id(&self, user_id: &str, with_user: &bool) -> Result<Option<Profile>>;

    /// Get or create a `Profile`.
    async fn get_or_create(
        &self,
        user_id: &str,
        input: &CreateProfileInput,
        with_user: &bool,
    ) -> Result<Profile>;

    /// Create a `Profile` with the given input
    async fn create(&self, input: &CreateProfileInput, with_user: &bool) -> Result<Profile>;

    /// Update an existing `Profile` by id
    async fn update(
        &self,
        id: &str,
        input: &UpdateProfileInput,
        with_user: &bool,
    ) -> Result<Profile>;

    /// Delete an existing `Profile`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `Service` struct
#[derive(new)]
pub struct DefaultService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

#[async_trait]
impl Service for DefaultService {
    async fn get(&self, id: &str, with_user: &bool) -> Result<Option<Profile>> {
        let query = model::Entity::find_by_id(id.to_owned());

        let profile = if *with_user {
            query
                .find_also_related(user_model::Entity)
                .one(&*self.db)
                .await?
        } else {
            query.one(&*self.db).await?.map(|u| (u, None))
        };

        let profile: ProfileOption = profile.into();

        Ok(profile.into())
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Profile>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let profiles = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        let profiles: ProfileList = profiles.into();

        Ok(profiles.into())
    }

    async fn get_many(
        &self,
        condition: Option<ProfileCondition>,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
        with_user: &bool,
    ) -> Result<ManyResponse<Profile>> {
        let page_num = page.unwrap_or(1);

        let mut query = model::Entity::find();

        if let Some(condition) = condition {
            if let Some(email) = condition.email {
                query = query.filter(model::Column::Email.eq(email));
            }

            if let Some(display_name) = condition.display_name {
                query = query.filter(model::Column::DisplayName.eq(display_name));
            }

            if let Some(city) = condition.city {
                query = query.filter(model::Column::City.eq(city));
            }

            if let Some(state_province) = condition.state_province {
                query = query.filter(model::Column::StateProvince.eq(state_province));
            }

            if let Some(user_id) = condition.user_id {
                query = query.filter(model::Column::UserId.eq(user_id));
            }

            if let Some(ids) = condition.ids_in {
                let mut condition = Condition::any();

                for id in ids {
                    condition = condition.add(model::Column::Id.eq(id.clone()));
                }

                query = query.filter(condition);
            }
        }

        if let Some(order_by) = order_by {
            for order in order_by {
                let ordering: Ordering<model::Column> = order.into();

                match ordering {
                    Ordering::Asc(column) => {
                        query = query.order_by_asc(column);
                    }
                    Ordering::Desc(column) => {
                        query = query.order_by_desc(column);
                    }
                }
            }
        }

        let (data, total) = match (page_size, with_user) {
            (Some(page_size), true) => {
                let paginator = query
                    .find_also_related(user_model::Entity)
                    .paginate(&*self.db, page_size);

                let total = paginator.num_items().await?;
                let data: ProfileList = paginator.fetch_page(page_num - 1).await?.into();

                (data, total)
            }
            (Some(page_size), false) => {
                let paginator = query.paginate(&*self.db, page_size);
                let total = paginator.num_items().await?;
                let data: ProfileList = paginator.fetch_page(page_num - 1).await?.into();

                (data, total)
            }
            (None, true) => {
                let data: ProfileList = query
                    .find_also_related(user_model::Entity)
                    .all(&*self.db)
                    .await?
                    .into();

                let total = data.len().try_into().unwrap_or(0);

                (data, total)
            }
            (None, false) => {
                let data: ProfileList = query.all(&*self.db).await?.into();
                let total = data.len().try_into().unwrap_or(0);

                (data, total)
            }
        };

        Ok(ManyResponse::new(data.into(), total, page_num, page_size))
    }

    async fn get_by_user_id(&self, user_id: &str, with_user: &bool) -> Result<Option<Profile>> {
        let query = model::Entity::find().filter(model::Column::UserId.eq(user_id.to_owned()));

        let profile: ProfileOption = match with_user {
            true => query
                .find_also_related(user_model::Entity)
                .one(&*self.db)
                .await?
                .into(),
            false => query.one(&*self.db).await?.into(),
        };

        Ok(profile.into())
    }

    async fn create(&self, input: &CreateProfileInput, with_user: &bool) -> Result<Profile> {
        let profile = model::ActiveModel {
            email: Set(input.email.clone()),
            display_name: Set(input.display_name.clone()),
            picture: Set(input.picture.clone()),
            city: Set(input.city.clone()),
            state_province: Set(input.state_province.clone()),
            user_id: Set(Some(input.user_id.clone())),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let mut created: Profile = profile.into();

        if !with_user {
            return Ok(created);
        }

        let user = user_model::Entity::find_by_id(input.user_id.clone())
            .one(&*self.db)
            .await?;

        created.user = user;

        Ok(created)
    }

    async fn get_or_create(
        &self,
        user_id: &str,
        input: &CreateProfileInput,
        with_user: &bool,
    ) -> Result<Profile> {
        let profile = self.get_by_user_id(user_id, with_user).await?;

        if let Some(profile) = profile {
            return Ok(profile);
        }

        self.create(input, with_user).await
    }

    async fn update(
        &self,
        id: &str,
        input: &UpdateProfileInput,
        with_user: &bool,
    ) -> Result<Profile> {
        let query = model::Entity::find_by_id(id.to_owned());

        // Pull out the `Profile` and the related `User`, if selected
        let (profile, user) = if *with_user {
            query
                .find_also_related(user_model::Entity)
                .one(&*self.db)
                .await?
        } else {
            // If the Profile isn't requested, just map to None
            query.one(&*self.db).await?.map(|p| (p, None))
        }
        .ok_or_else(|| anyhow!("Unable to find Profile with id: {}", id))?;

        let mut profile: model::ActiveModel = profile.into();

        if let Some(email) = &input.email {
            profile.email = Set(email.clone());
        }

        match &input.display_name {
            Undefined => (),
            Null => profile.display_name = Set(None),
            Value(value) => profile.display_name = Set(Some(value.clone())),
        }

        match &input.picture {
            Undefined => (),
            Null => profile.picture = Set(None),
            Value(value) => profile.picture = Set(Some(value.clone())),
        }

        match &input.city {
            Undefined => (),
            Null => profile.city = Set(None),
            Value(value) => profile.city = Set(Some(value.clone())),
        }

        match &input.state_province {
            Undefined => (),
            Null => profile.state_province = Set(None),
            Value(value) => profile.state_province = Set(Some(value.clone())),
        }

        if let Some(user_id) = &input.user_id {
            profile.user_id = Set(Some(user_id.clone()));
        }

        let mut updated: Profile = profile.update(&*self.db).await?.into();

        // Add back the User from above
        updated.user = user;

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let profile = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Profile with id: {}", id))?;

        let _result = profile.delete(&*self.db).await?;

        Ok(())
    }
}

/// Provide the Service
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Box<dyn Service>> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Box<dyn Service>>> {
        let db = i.get::<DatabaseConnection>().await?;

        Ok(Arc::new(Box::new(DefaultService::new(db))))
    }
}
