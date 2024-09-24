use std::sync::Arc;

use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use async_trait::async_trait;
use derive_new::new;
use hyper::StatusCode;
use nakago::{provider, Inject, Provider};
use nakago_async_graphql::utils::{as_graphql_error, graphql_error};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;

use crate::domains::profiles::{self, mutation::CreateProfileInput};

use super::{model::User, Service};

/// The `CreateUserProfileInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct CreateUserProfileInput {
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
}

/// The `CreateUserInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct CreateUserInput {
    /// The User's profile
    pub profile: Option<CreateUserProfileInput>,
}

/// The `UpdateUserInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateUserInput {
    /// The User's subscriber id
    pub username: Option<String>,

    /// Whether the User is active or disabled
    pub is_active: Option<bool>,
}

/// The `MutateUserResult` input type
#[derive(Clone, Default, Eq, PartialEq, SimpleObject)]
pub struct MutateUserResult {
    /// The User's subscriber id
    pub user: Option<User>,
}

/// The Mutation segment for Users
#[derive(new)]
pub struct UsersMutation {
    service: Arc<Box<dyn Service>>,
    profiles: Arc<Box<dyn profiles::Service>>,
}

/// Mutations for the User model
#[Object]
impl UsersMutation {
    /// Get or create the current User based on the current token username (the "sub" claim)
    async fn get_or_create_current_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<MutateUserResult> {
        let user = ctx.data_unchecked::<Option<User>>();
        let subject = ctx.data_unchecked::<Subject>();

        // If the User exists in the GraphQL context, simply return it
        if let Some(user) = user {
            return Ok(MutateUserResult {
                user: Some(user.clone()),
            });
        }

        // Otherwise, check for a username so that it can be created
        let username = match subject {
            Subject {
                username: Some(username),
                ..
            } => Ok(username),
            _ => Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED)),
        }?;

        let user = self
            .service
            .create(username)
            .await
            .map_err(as_graphql_error(
                "Eror while creating User",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        if let Some(profile) = input.profile {
            self.profiles
                .get_or_create(
                    &user.id,
                    &CreateProfileInput {
                        email: profile.email,
                        display_name: profile.display_name,
                        picture: profile.picture,
                        city: profile.city,
                        state_province: profile.state_province,
                        user_id: user.id.clone(),
                    },
                    &false,
                )
                .await?;
        }

        Ok(MutateUserResult { user: Some(user) })
    }

    /// Update the current User based on the current token username (the "sub" claim)
    async fn update_current_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInput,
    ) -> Result<MutateUserResult> {
        let user = ctx.data_unchecked::<Option<User>>();

        // Check to see if the associated Profile is selected
        let with_roles = ctx.look_ahead().field("user").field("roles").exists();

        if let Some(user) = user {
            let updated = self
                .service
                .update(&user.id, &input, &with_roles)
                .await
                .map_err(as_graphql_error(
                    "Error while updating User",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))?;

            return Ok(MutateUserResult {
                user: Some(updated),
            });
        }

        Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED))
    }
}

/// Provide the UsersMutation
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<UsersMutation> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<UsersMutation>> {
        let service = i.get::<Box<dyn Service>>().await?;
        let profiles = i.get::<Box<dyn profiles::Service>>().await?;

        Ok(Arc::new(UsersMutation::new(service, profiles)))
    }
}
