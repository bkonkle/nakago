use anyhow::Result;
use async_graphql::MaybeUndefined::{Null, Undefined, Value};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use nakago::{Inject, InjectResult, Provide, Tag};
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};
use std::sync::Arc;

use crate::utils::{ordering::Ordering, pagination::ManyResponse};
use crate::{
    db::provider::DATABASE_CONNECTION,
    domains::shows::{
        model::{self, Show},
        mutations::{CreateShowInput, UpdateShowInput},
        queries::{ShowCondition, ShowsOrderBy},
    },
};

/// Tag(ShowsService)
pub const SHOWS_SERVICE: Tag<Arc<dyn Service>> = Tag::new("ShowsService");

/// Provide the Shows Service
///
/// **Provides:** `Arc<dyn Service>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct Provider {}

#[async_trait]
impl Provide<Arc<dyn Service>> for Provider {
    async fn provide(&self, i: &Inject) -> InjectResult<Arc<dyn Service>> {
        let db = i.get(&DATABASE_CONNECTION)?;

        Ok(Arc::new(DefaultService::new(db.clone())))
    }
}

/// A Service applies business logic to a dynamic ShowsRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Service: Sync + Send {
    /// Get an individual `Show` by id
    async fn get(&self, id: &str) -> Result<Option<Show>>;

    /// Get a list of `Show` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Show>>;

    /// Get multiple `Show` records
    async fn get_many(
        &self,
        condition: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ManyResponse<Show>>;

    /// Create a `Show` with the given input
    async fn create(&self, input: &CreateShowInput) -> Result<Show>;

    /// Update an existing `Show` by id
    async fn update(&self, id: &str, input: &UpdateShowInput) -> Result<Show>;

    /// Delete an existing `Show`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `Service` struct.
pub struct DefaultService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `Service` implementation
impl DefaultService {
    /// Create a new `Service` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Service for DefaultService {
    async fn get(&self, id: &str) -> Result<Option<model::Model>> {
        let query = model::Entity::find_by_id(id.to_owned());

        let show = query.one(&*self.db).await?;

        Ok(show)
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Show>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let shows = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(shows)
    }

    async fn get_many(
        &self,
        condition: Option<ShowCondition>,
        order_by: Option<Vec<ShowsOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<ManyResponse<Show>> {
        let page_num = page.unwrap_or(1);

        let mut query = model::Entity::find();

        if let Some(condition) = condition {
            if let Some(title) = condition.title {
                query = query.filter(model::Column::Title.eq(title));
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

        let (data, total) = if let Some(page_size) = page_size {
            let paginator = query.paginate(&*self.db, page_size);
            let total = paginator.num_items().await?;
            let data: Vec<Show> = paginator.fetch_page(page_num - 1).await?;

            (data, total)
        } else {
            let data: Vec<Show> = query.all(&*self.db).await?;
            let total = data.len().try_into().unwrap_or(0);

            (data, total)
        };

        Ok(ManyResponse::new(data, total, page_num, page_size))
    }

    async fn create(&self, input: &CreateShowInput) -> Result<Show> {
        let show = model::ActiveModel {
            title: Set(input.title.clone()),
            summary: Set(input.summary.clone()),
            picture: Set(input.picture.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let created: Show = show;

        return Ok(created);
    }

    async fn update(&self, id: &str, input: &UpdateShowInput) -> Result<Show> {
        let query = model::Entity::find_by_id(id.to_owned());

        // Retrieve the existing Show
        let show = query
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

        let mut show: model::ActiveModel = show.into();

        match &input.title {
            Undefined | Null => (),
            Value(value) => show.title = Set(value.clone()),
        };

        match &input.summary {
            Undefined => (),
            Null => show.summary = Set(None),
            Value(value) => show.summary = Set(Some(value.clone())),
        }

        match &input.picture {
            Undefined => (),
            Null => show.picture = Set(None),
            Value(value) => show.picture = Set(Some(value.clone())),
        }

        let updated: Show = show.update(&*self.db).await?;

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let show = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Show with id: {}", id))?;

        let _result = show.delete(&*self.db).await?;

        Ok(())
    }
}
