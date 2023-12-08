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

use super::{
    model::{self, Episode},
    Service, SERVICE,
};

use EpisodesOrderBy::{
    CreatedAtAsc, CreatedAtDesc, IdAsc, IdDesc, ShowIdAsc, ShowIdDesc, TitleAsc, TitleDesc,
    UpdatedAtAsc, UpdatedAtDesc,
};

/// Tag(episodes::Query)
pub const QUERY: Tag<EpisodesQuery> = Tag::new("episodes::Query");

/// The `EpisodesPage` result type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct EpisodesPage {
    /// The list of `Episodes` returned for the current page
    data: Vec<Episode>,

    /// The number of `Episodes` returned for the current page
    count: u64,

    /// Tne total number of `Episodes` available
    total: u64,

    /// The current page
    page: u64,

    /// The number of pages available
    page_count: u64,
}

impl From<ManyResponse<Episode>> for EpisodesPage {
    fn from(resp: ManyResponse<Episode>) -> EpisodesPage {
        EpisodesPage {
            data: resp.data,
            count: resp.count,
            total: resp.total,
            page: resp.page,
            page_count: resp.page_count,
        }
    }
}

/// Conditions to filter Episode listings by
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct EpisodeCondition {
    /// The `Episode`'s title
    pub title: Option<String>,

    /// The associated Show
    pub show_id: Option<String>,

    /// Filter by IDs
    pub ids_in: Option<Vec<String>>,
}

/// The available ordering values
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum EpisodesOrderBy {
    /// Order ascending by "id"
    IdAsc,
    /// Order descending by "id"
    IdDesc,
    /// Order ascending by "displayName"
    TitleAsc,
    /// Order descending by "displayName"
    TitleDesc,
    /// Order ascending by "showId"
    ShowIdAsc,
    /// Order descending by "showId"
    ShowIdDesc,
    /// Order ascending by "createdAt"
    CreatedAtAsc,
    /// Order descending by "createdAt"
    CreatedAtDesc,
    /// Order ascending by "updatedAt"
    UpdatedAtAsc,
    /// Order descending by "updatedAt"
    UpdatedAtDesc,
}

impl From<EpisodesOrderBy> for Ordering<model::Column> {
    fn from(order_by: EpisodesOrderBy) -> Ordering<model::Column> {
        match order_by {
            IdAsc => Asc(model::Column::Id),
            TitleAsc => Asc(model::Column::Title),
            ShowIdAsc => Asc(model::Column::ShowId),
            CreatedAtAsc => Asc(model::Column::CreatedAt),
            UpdatedAtAsc => Asc(model::Column::UpdatedAt),
            IdDesc => Desc(model::Column::Id),
            TitleDesc => Desc(model::Column::Title),
            ShowIdDesc => Desc(model::Column::ShowId),
            CreatedAtDesc => Desc(model::Column::CreatedAt),
            UpdatedAtDesc => Desc(model::Column::UpdatedAt),
        }
    }
}

/// The Query segment owned by the Episodes library
#[derive(new)]
pub struct EpisodesQuery {
    service: Arc<Box<dyn Service>>,
}

/// Queries for the `Episode` model
#[Object]
impl EpisodesQuery {
    /// Get a sincle Episode
    pub async fn get_episode(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The Episode id")] id: String,
    ) -> Result<Option<Episode>> {
        // Check to see if the associated Show is selected
        let with_show = ctx.look_ahead().field("show").exists();

        self.service
            .get(&id, &with_show)
            .await
            .map_err(as_graphql_error(
                "Error while retrieving Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
    }

    /// Get multiple Episodes
    pub async fn get_many_episodes(
        &self,
        ctx: &Context<'_>,
        r#where: Option<EpisodeCondition>,
        order_by: Option<Vec<EpisodesOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<EpisodesPage> {
        // Check to see if the associated Show is selected
        let with_show = ctx.look_ahead().field("data").field("show").exists();

        let response = self
            .service
            .get_many(r#where, order_by, page, page_size, &with_show)
            .await
            .map_err(as_graphql_error(
                "Error while listing Episodes",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(response.into())
    }
}

/// Provide the EpisodesQuery
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<EpisodesQuery> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<EpisodesQuery>> {
        let service = i.get(&SERVICE).await?;

        Ok(Arc::new(EpisodesQuery::new(service)))
    }
}
