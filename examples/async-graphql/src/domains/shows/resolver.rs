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

use super::{
    model::Show,
    mutations::{CreateShowInput, MutateShowResult, UpdateShowInput},
    queries::{ShowCondition, ShowsOrderBy, ShowsPage},
    Service,
};

/// The Query segment owned by the Shows library
#[derive(new)]
pub struct ShowsQuery {
    service: Arc<Box<dyn Service>>,
}

/// Queries for the `Show` model
#[Object]
impl ShowsQuery {
    async fn get_show(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "The Show id")] id: String,
    ) -> Result<Option<Show>> {
        Ok(self.service.get(&id).await?)
    }

    /// Get multiple Shows
    async fn get_many_shows(
        &self,
        _ctx: &Context<'_>,
        r#where: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ShowsPage> {
        let response = self
            .service
            .get_many(r#where, order_by, page, page_size)
            .await
            .map_err(as_graphql_error(
                "Error while listing Shows",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(response.into())
    }
}

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
