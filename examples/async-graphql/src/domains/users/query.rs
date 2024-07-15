use std::sync::Arc;

use async_graphql::{Context, Object, Result};
use async_trait::async_trait;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;

use super::model::User;

/// The Query segment for Users
#[derive(Default)]
pub struct UsersQuery {}

/// Queries for the User model
#[Object]
impl UsersQuery {
    /// Get the current User from the GraphQL context
    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let user = ctx.data_unchecked::<Option<User>>();

        Ok(user.clone())
    }
}

/// Provide the UsersQuery
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<UsersQuery> for Provide {
    async fn provide(self: Arc<Self>, _: Inject) -> provider::Result<Arc<UsersQuery>> {
        Ok(Arc::new(UsersQuery::default()))
    }
}
