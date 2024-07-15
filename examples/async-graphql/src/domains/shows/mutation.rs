use async_graphql::{InputObject, MaybeUndefined, SimpleObject};
use async_trait::async_trait;
use fake::{Dummy, Faker};
use nakago::{provider, Inject, Provider};
use nakago_async_graphql::utils::dummy_maybe_undef;
use nakago_derive::Provider;
use rand::Rng;

use super::{model::Show, Service};

/// The `CreateShowInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateShowInput {
    /// The Show's title
    pub title: String,

    /// The Show's description summary
    pub summary: Option<String>,

    /// The Show's picture
    pub picture: Option<String>,
}

/// The `UpdateShowInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateShowInput {
    /// The Show's title
    pub title: MaybeUndefined<String>,

    /// The Show's description summary
    pub summary: MaybeUndefined<String>,

    /// The Show's picture
    pub picture: MaybeUndefined<String>,
}

impl Dummy<Faker> for UpdateShowInput {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        UpdateShowInput {
            title: dummy_maybe_undef(config, rng),
            summary: dummy_maybe_undef(config, rng),
            picture: dummy_maybe_undef(config, rng),
        }
    }
}

/// The `MutateShowResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateShowResult {
    /// The Show's subscriber id
    pub show: Option<Show>,
}

use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use derive_new::new;
use hyper::StatusCode;
use nakago_async_graphql::utils::{as_graphql_error, graphql_error};
use oso::Oso;

use crate::domains::{
    role_grants::{self, model::CreateRoleGrantInput},
    users::model::User,
};

/// The Mutation segment for Shows
#[derive(new)]
pub struct ShowsMutation {
    service: Arc<Box<dyn Service>>,
    role_grants: Arc<Box<dyn role_grants::Service>>,
}

/// Mutations for the Show model
#[Object]
impl ShowsMutation {
    /// Create a new Show
    async fn create_show(
        &self,
        ctx: &Context<'_>,
        input: CreateShowInput,
    ) -> Result<MutateShowResult> {
        let user = ctx.data_unchecked::<Option<User>>();

        // Check authorization
        if let Some(user) = user {
            let show = self.service.create(&input).await.map_err(as_graphql_error(
                "Error while creating Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

            // Grant the Admin role to the creator
            self.role_grants
                .create(&CreateRoleGrantInput {
                    role_key: "admin".to_string(),
                    user_id: user.id.clone(),
                    resource_table: "shows".to_string(),
                    resource_id: show.id.clone(),
                })
                .await
                .map_err(as_graphql_error(
                    "Error while granting the admin role for a Show",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))?;

            Ok(MutateShowResult { show: Some(show) })
        } else {
            Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED))
        }
    }

    /// Update an existing Show
    async fn update_show(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateShowInput,
    ) -> Result<MutateShowResult> {
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the existing Show for authorization
        let existing = self
            .service
            .get(&id)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "update", existing)? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        let show = self
            .service
            .update(&id, &input)
            .await
            .map_err(as_graphql_error(
                "Error while updating Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateShowResult { show: Some(show) })
    }

    /// Remove an existing Show
    async fn delete_show(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the existing Show for authorization
        let existing = self
            .service
            .get(&id)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "delete", existing)? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        self.service.delete(&id).await.map_err(as_graphql_error(
            "Error while deleting Show",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(true)
    }
}

/// Provide the ShowsMutation
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<ShowsMutation> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<ShowsMutation>> {
        let service = i.get::<Box<dyn Service>>().await?;
        let role_grants = i.get::<Box<dyn role_grants::Service>>().await?;

        Ok(Arc::new(ShowsMutation::new(service, role_grants)))
    }
}
