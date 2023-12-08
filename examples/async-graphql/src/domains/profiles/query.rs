use std::sync::Arc;

use async_graphql::{Context, Enum, InputObject, Object, Result, SimpleObject};
use async_trait::async_trait;
use derive_new::new;
use hyper::StatusCode;
use nakago::{provider, Inject, Provider, Tag};
use nakago_async_graphql::utils::as_graphql_error;
use nakago_axum::utils::{
    ManyResponse,
    Ordering::{self, Asc, Desc},
};
use nakago_derive::Provider;

use crate::domains::users::model::User;

use super::{
    model::{self, Profile},
    Service, SERVICE,
};

use ProfilesOrderBy::{
    CreatedAtAsc, CreatedAtDesc, DisplayNameAsc, DisplayNameDesc, EmailAsc, EmailDesc, IdAsc,
    IdDesc, UpdatedAtAsc, UpdatedAtDesc,
};

/// Tag(profiles::Query)
pub const QUERY: Tag<ProfilesQuery> = Tag::new("profiles::Query");

/// The `ProfilesPage` result type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct ProfilesPage {
    /// The list of `Profiles` returned for the current page
    data: Vec<Profile>,

    /// The number of `Profiles` returned for the current page
    count: u64,

    /// Tne total number of `Profiles` available
    total: u64,

    /// The current page
    page: u64,

    /// The number of pages available
    page_count: u64,
}

impl From<ManyResponse<Profile>> for ProfilesPage {
    fn from(resp: ManyResponse<Profile>) -> ProfilesPage {
        ProfilesPage {
            data: resp.data,
            count: resp.count,
            total: resp.total,
            page: resp.page,
            page_count: resp.page_count,
        }
    }
}

/// Conditions to filter Profile listings by
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct ProfileCondition {
    /// The `Profile`'s email address
    pub email: Option<String>,

    /// The `Profile`'s display name
    pub display_name: Option<String>,

    /// The `Profile`'s city
    pub city: Option<String>,

    /// The `Profile`'s state or province
    pub state_province: Option<String>,

    /// The `Profile`'s User id
    pub user_id: Option<String>,

    /// Filter by IDs
    pub ids_in: Option<Vec<String>>,
}

/// The available ordering values
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ProfilesOrderBy {
    /// Order ascending by "id"
    IdAsc,
    /// Order descending by "id"
    IdDesc,
    /// Order ascending by "email"
    EmailAsc,
    /// Order descending by "email"
    EmailDesc,
    /// Order ascending by "displayName"
    DisplayNameAsc,
    /// Order descending by "displayName"
    DisplayNameDesc,
    /// Order ascending by "createdAt"
    CreatedAtAsc,
    /// Order descending by "createdAt"
    CreatedAtDesc,
    /// Order ascending by "updatedAt"
    UpdatedAtAsc,
    /// Order descending by "updatedAt"
    UpdatedAtDesc,
}

impl From<ProfilesOrderBy> for Ordering<model::Column> {
    fn from(order_by: ProfilesOrderBy) -> Ordering<model::Column> {
        match order_by {
            IdAsc => Asc(model::Column::Id),
            EmailAsc => Asc(model::Column::Email),
            DisplayNameAsc => Asc(model::Column::DisplayName),
            CreatedAtAsc => Asc(model::Column::CreatedAt),
            UpdatedAtAsc => Asc(model::Column::UpdatedAt),
            IdDesc => Desc(model::Column::Id),
            EmailDesc => Desc(model::Column::Email),
            DisplayNameDesc => Desc(model::Column::DisplayName),
            CreatedAtDesc => Desc(model::Column::CreatedAt),
            UpdatedAtDesc => Desc(model::Column::UpdatedAt),
        }
    }
}

/// The Query segment for Profiles
#[derive(new)]
pub struct ProfilesQuery {
    service: Arc<Box<dyn Service>>,
}

/// Queries for the `Profile` model
#[Object]
impl ProfilesQuery {
    /// Get a single Profile
    async fn get_profile(&self, ctx: &Context<'_>, id: String) -> Result<Option<Profile>> {
        let user = ctx.data_unchecked::<Option<User>>();

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("user").exists();

        let profile = self.service.get(&id, &with_user).await?;

        // Use the request User to decide if the Profile should be censored
        let censored = match user {
            Some(user) => {
                let user_id = user.id.clone();

                // If the User and Profile are present, censor the Profile based on the User id
                profile.map(|p| {
                    Profile {
                        user: Some(user.clone()),
                        ..p
                    }
                    .censor(&Some(user_id))
                })
            }
            // If the User is absent, always censor the Profile
            None => profile.map(|p| p.censor(&None)),
        };

        Ok(censored)
    }

    /// Get multiple Profiles
    async fn get_many_profiles(
        &self,
        ctx: &Context<'_>,
        r#where: Option<ProfileCondition>,
        order_by: Option<Vec<ProfilesOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ProfilesPage> {
        let user = ctx.data_unchecked::<Option<User>>();

        // Retrieve the current request User id for authorization
        let user_id = user.clone().map(|u| u.id);

        // Check to see if the associated User is selected
        let with_user = ctx.look_ahead().field("data").field("user").exists();

        let response = self
            .service
            .get_many(r#where, order_by, page, page_size, &with_user)
            .await
            .map_err(as_graphql_error(
                "Error while listing Profiles",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        let censored = response.map(|p| p.censor(&user_id));

        Ok(censored.into())
    }
}

/// Provide the ProfilesQuery
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<ProfilesQuery> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<ProfilesQuery>> {
        let service = i.get(&SERVICE).await?;

        Ok(Arc::new(ProfilesQuery::new(service)))
    }
}
