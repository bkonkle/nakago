use async_graphql::{Context, Object, Result};
use async_trait::async_trait;
use hyper::StatusCode;
use nakago::{Hook, Inject, InjectResult};
use nakago_axum::auth::Subject;
use std::sync::Arc;

use super::{
    model::User,
    mutations::{CreateUserInput, MutateUserResult, UpdateUserInput},
    service::{UsersService, USERS_SERVICE},
};
use crate::{
    domains::profiles::{
        mutations::CreateProfileInput,
        service::{ProfilesService, PROFILES_SERVICE},
    },
    graphql::GRAPHQL_SCHEMA_BUILDER,
    utils::graphql::{as_graphql_error, graphql_error},
};

/// The Query segment for Users
#[derive(Default)]
pub struct UsersQuery {}

/// The Mutation segment for Users
#[derive(Default)]
pub struct UsersMutation {}

/// Queries for the User model
#[Object]
impl UsersQuery {
    /// Get the current User from the GraphQL context
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let user = ctx.data_unchecked::<Option<User>>();

        Ok(user.clone())
    }
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
        let users = ctx.data_unchecked::<Arc<Box<dyn UsersService>>>();
        let profiles = ctx.data_unchecked::<Arc<Box<dyn ProfilesService>>>();
        let subject = ctx.data_unchecked::<Subject>();

        // If the User exists in the GraphQL context, simply return it
        if let Some(user) = user {
            return Ok(MutateUserResult {
                user: Some(user.clone()),
            });
        }

        // Otherwise, check for a username so that it can be created
        let username = match subject {
            Subject(Some(username)) => Ok(username),
            _ => Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED)),
        }?;

        let user = users.create(username).await.map_err(as_graphql_error(
            "Eror while creating User",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        if let Some(profile) = input.profile {
            profiles
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
        let users = ctx.data_unchecked::<Arc<Box<dyn UsersService>>>();

        // Check to see if the associated Profile is selected
        let with_roles = ctx.look_ahead().field("user").field("roles").exists();

        if let Some(user) = user {
            let updated =
                users
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

/// The Hook for initializing the Users resolver
#[derive(Default)]
pub struct InitGraphQLUsers {}

#[async_trait]
impl Hook for InitGraphQLUsers {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        let users = i.get(&USERS_SERVICE).await?;
        let profiles = i.get(&PROFILES_SERVICE).await?;

        let builder = i.consume(&GRAPHQL_SCHEMA_BUILDER).await?;

        i.inject(
            &GRAPHQL_SCHEMA_BUILDER,
            builder.data(users.clone()).data(profiles.clone()),
        )
        .await?;

        Ok(())
    }
}
