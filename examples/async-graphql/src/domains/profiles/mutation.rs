use std::sync::Arc;

use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, MaybeUndefined, Object, Result,
    SimpleObject,
};
use async_trait::async_trait;
use derive_new::new;
use fake::{faker::internet::en::FreeEmail, Dummy, Fake, Faker};
use hyper::StatusCode;
use nakago::{provider, Inject, Provider};
use nakago_async_graphql::utils::{as_graphql_error, dummy_maybe_undef, graphql_error};
use nakago_derive::Provider;
use rand::Rng;

use crate::domains::users::{self, model::User};

use super::{model::Profile, Service};

/// The `CreateProfileInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateProfileInput {
    /// The Profile's email address
    pub email: String,

    /// The Profile's display name
    pub display_name: Option<String>,

    /// The Profile's picture
    pub picture: Option<String>,

    /// The Profile's city
    pub city: Option<String>,

    /// The Profile's state or province
    pub state_province: Option<String>,

    /// The Profile's User id
    pub user_id: String,
}

/// The `UpdateProfileInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateProfileInput {
    /// The Profile's email address
    pub email: Option<String>,

    /// The Profile's display name
    pub display_name: MaybeUndefined<String>,

    /// The Profile's picture
    pub picture: MaybeUndefined<String>,

    /// The Profile's city
    pub city: MaybeUndefined<String>,

    /// The Profile's state or province
    pub state_province: MaybeUndefined<String>,

    /// The Profile's User id
    pub user_id: Option<String>,
}

impl Dummy<Faker> for UpdateProfileInput {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        UpdateProfileInput {
            email: FreeEmail().fake(),
            display_name: dummy_maybe_undef(config, rng),
            picture: dummy_maybe_undef(config, rng),
            city: dummy_maybe_undef(config, rng),
            state_province: dummy_maybe_undef(config, rng),
            user_id: Faker.fake(),
        }
    }
}

/// The `MutateProfileResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateProfileResult {
    /// The Profile's subscriber id
    pub profile: Option<Profile>,
}

/// The Mutation segment for Profiles
#[derive(new)]
pub struct ProfilesMutation {
    service: Arc<Box<dyn Service>>,
}

/// Mutations for the Profile model
#[Object]
impl ProfilesMutation {
    /// Create a new Profile
    async fn create_profile(
        &self,
        ctx: &Context<'_>,
        input: CreateProfileInput,
    ) -> Result<MutateProfileResult> {
        let user = ctx.data_unchecked::<Option<User>>();

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        if let Some(user_id) = user_id {
            // Make sure the current request User id matches the input
            if user_id != input.user_id {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            // If there is no request User, return a 401
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("profile").field("user").exists();

        let profile = self
            .service
            .create(&input, &with_user)
            .await
            .map_err(as_graphql_error(
                "Error while creating Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateProfileResult {
            profile: Some(profile),
        })
    }

    /// Update an existing Profile
    async fn update_profile(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateProfileInput,
    ) -> Result<MutateProfileResult> {
        let user = ctx.data_unchecked::<Option<User>>();

        // Retrieve the existing Profile for authorization
        let existing = self
            .service
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Profile", StatusCode::NOT_FOUND)
            })?;

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        if let Some(user_id) = user_id {
            // Make sure the current request User id matches the existing user
            if existing.user.as_ref().map(|u| u.id.clone()) != Some(user_id) {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            // If there is no request User, return a 401
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("profile").field("user").exists();

        // Use the already retrieved Profile to update the record
        let profile = self
            .service
            .update(&existing.id, &input, &with_user)
            .await
            .map_err(as_graphql_error(
                "Error while updating Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateProfileResult {
            profile: Some(profile),
        })
    }

    /// Remove an existing Profile
    async fn delete_profile(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let user = ctx.data_unchecked::<Option<User>>();

        // Retrieve the existing Profile for authorization
        let existing = self
            .service
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Profile", StatusCode::NOT_FOUND)
            })?;

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        if let Some(user_id) = user_id {
            // Make sure the current request User id matches the existing user
            if existing.user.as_ref().map(|u| u.id.clone()) != Some(user_id) {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            // If there is no request User, return a 401
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        self.service.delete(&id).await.map_err(as_graphql_error(
            "Error while deleting Profile",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(true)
    }
}

#[ComplexObject]
impl Profile {
    #[graphql(name = "user")]
    async fn resolve_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        if let Some(user) = self.user.clone() {
            return Ok(Some(user));
        }

        if let Some(user_id) = self.user_id.clone() {
            let loader = ctx.data_unchecked::<DataLoader<users::Loader>>();
            let user = loader.load_one(user_id).await?;

            return Ok(user);
        }

        Ok(None)
    }
}

/// Provide the ProfilesMutation
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<ProfilesMutation> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<ProfilesMutation>> {
        let service = i.get::<Box<dyn Service>>().await?;

        Ok(Arc::new(ProfilesMutation::new(service)))
    }
}
