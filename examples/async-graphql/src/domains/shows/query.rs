use std::sync::Arc;

use async_graphql::{Context, Enum, InputObject, Object, Result, SimpleObject};
use async_trait::async_trait;
use derive_new::new;
use hyper::StatusCode;
use nakago::{provider, Inject, Provider};
use nakago_async_graphql::utils::as_graphql_error;
use nakago_axum::utils::{
    ManyResponse,
    Ordering::{self, Asc, Desc},
};
use nakago_derive::Provider;

use super::{
    model::{self, Show},
    Service,
};

use ShowsOrderBy::{
    CreatedAtAsc, CreatedAtDesc, IdAsc, IdDesc, TitleAsc, TitleDesc, UpdatedAtAsc, UpdatedAtDesc,
};

/// The `ShowsPage` result type
#[derive(Clone, Eq, PartialEq, SimpleObject)]
pub struct ShowsPage {
    /// The list of `Shows` returned for the current page
    data: Vec<Show>,

    /// The number of `Shows` returned for the current page
    count: u64,

    /// Tne total number of `Shows` available
    total: u64,

    /// The current page
    page: u64,

    /// The number of pages available
    page_count: u64,
}

impl From<ManyResponse<Show>> for ShowsPage {
    fn from(resp: ManyResponse<Show>) -> ShowsPage {
        ShowsPage {
            data: resp.data,
            count: resp.count,
            total: resp.total,
            page: resp.page,
            page_count: resp.page_count,
        }
    }
}

/// Conditions to filter Show listings by
#[derive(Clone, Eq, PartialEq, InputObject)]
pub struct ShowCondition {
    /// The `Show`'s title
    pub title: Option<String>,

    /// Filter by IDs
    pub ids_in: Option<Vec<String>>,
}

/// The available ordering values
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ShowsOrderBy {
    /// Order ascending by "id"
    IdAsc,
    /// Order descending by "id"
    IdDesc,
    /// Order ascending by "title"
    TitleAsc,
    /// Order descending by "title"
    TitleDesc,
    /// Order ascending by "createdAt"
    CreatedAtAsc,
    /// Order descending by "createdAt"
    CreatedAtDesc,
    /// Order ascending by "updatedAt"
    UpdatedAtAsc,
    /// Order descending by "updatedAt"
    UpdatedAtDesc,
}

impl From<ShowsOrderBy> for Ordering<model::Column> {
    fn from(order_by: ShowsOrderBy) -> Ordering<model::Column> {
        match order_by {
            IdAsc => Asc(model::Column::Id),
            TitleAsc => Asc(model::Column::Title),
            CreatedAtAsc => Asc(model::Column::CreatedAt),
            UpdatedAtAsc => Asc(model::Column::UpdatedAt),
            IdDesc => Desc(model::Column::Id),
            TitleDesc => Desc(model::Column::Title),
            CreatedAtDesc => Desc(model::Column::CreatedAt),
            UpdatedAtDesc => Desc(model::Column::UpdatedAt),
        }
    }
}

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

/// Provide the ShowsQuery
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<ShowsQuery> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<ShowsQuery>> {
        let service = i.get_type::<Box<dyn Service>>().await?;

        Ok(Arc::new(ShowsQuery::new(service)))
    }
}
