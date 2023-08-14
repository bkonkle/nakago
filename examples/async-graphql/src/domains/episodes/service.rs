use std::sync::Arc;

use anyhow::Result;
use async_graphql::MaybeUndefined::{Null, Undefined, Value};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use nakago::{Dependency, Inject, InjectResult, Provider, Tag};
use sea_orm::{entity::*, query::*, DatabaseConnection, EntityTrait};

use super::{
    model::{self, Episode, EpisodeList, EpisodeOption},
    mutations::{CreateEpisodeInput, UpdateEpisodeInput},
    queries::{EpisodeCondition, EpisodesOrderBy},
};
use crate::{
    db::DATABASE_CONNECTION,
    domains::shows::model as show_model,
    utils::{ordering::Ordering, pagination::ManyResponse},
};

/// Tag(EpisodesService)
pub const EPISODES_SERVICE: Tag<Box<dyn EpisodesService>> = Tag::new("EpisodesService");

/// An EpisodesService applies business logic to a dynamic EpisodesRepository implementation.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait EpisodesService: Sync + Send {
    /// Get an individual `Episode` by id
    async fn get(&self, id: &str, with_show: &bool) -> Result<Option<Episode>>;

    /// Get a list of `Episode` results matching the given ids
    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Episode>>;

    /// Get multiple `Episode` records
    async fn get_many(
        &self,
        condition: Option<EpisodeCondition>,
        order_by: Option<Vec<EpisodesOrderBy>>,
        page_size: Option<u64>,
        page: Option<u64>,
        with_show: &bool,
    ) -> Result<ManyResponse<Episode>>;

    /// Create a `Episode` with the given input
    async fn create(&self, input: &CreateEpisodeInput, with_show: &bool) -> Result<Episode>;

    /// Update an existing `Episode` by id
    async fn update(
        &self,
        id: &str,
        input: &UpdateEpisodeInput,
        with_show: &bool,
    ) -> Result<Episode>;

    /// Delete an existing `Episode`
    async fn delete(&self, id: &str) -> Result<()>;
}

/// The default `EpisodesService` struct.
pub struct DefaultEpisodesService {
    /// The SeaOrm database connection
    db: Arc<DatabaseConnection>,
}

/// The default `EpisodesService` implementation
impl DefaultEpisodesService {
    /// Create a new `EpisodesService` instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EpisodesService for DefaultEpisodesService {
    async fn get(&self, id: &str, with_show: &bool) -> Result<Option<Episode>> {
        let query = model::Entity::find_by_id(id.to_owned());

        let episode = if *with_show {
            query
                .find_also_related(show_model::Entity)
                .one(&*self.db)
                .await?
        } else {
            query.one(&*self.db).await?.map(|u| (u, None))
        };

        let episode: EpisodeOption = episode.into();

        Ok(episode.into())
    }

    async fn get_by_ids(&self, ids: Vec<String>) -> Result<Vec<Episode>> {
        let mut condition = Condition::any();

        for id in ids {
            condition = condition.add(model::Column::Id.eq(id.clone()));
        }

        let episodes = model::Entity::find()
            .filter(condition)
            .all(&*self.db)
            .await?;

        Ok(episodes)
    }

    async fn get_many(
        &self,
        condition: Option<EpisodeCondition>,
        order_by: Option<Vec<EpisodesOrderBy>>,
        page: Option<u64>,
        page_size: Option<u64>,
        with_show: &bool,
    ) -> Result<ManyResponse<Episode>> {
        let page_num = page.unwrap_or(1);

        let mut query = model::Entity::find();

        if let Some(condition) = condition {
            if let Some(title) = condition.title {
                query = query.filter(model::Column::Title.eq(title));
            }

            if let Some(show_id) = condition.show_id {
                query = query.filter(model::Column::ShowId.eq(show_id));
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

        let (data, total) = match (page_size, with_show) {
            (Some(page_size), true) => {
                let paginator = query
                    .find_also_related(show_model::Entity)
                    .paginate(&*self.db, page_size);

                let total = paginator.num_items().await?;
                let data: EpisodeList = paginator.fetch_page(page_num - 1).await?.into();

                (data, total)
            }
            (Some(page_size), false) => {
                let paginator = query.paginate(&*self.db, page_size);
                let total = paginator.num_items().await?;
                let data: EpisodeList = paginator.fetch_page(page_num - 1).await?.into();

                (data, total)
            }
            (None, true) => {
                let data: EpisodeList = query
                    .find_also_related(show_model::Entity)
                    .all(&*self.db)
                    .await?
                    .into();

                let total = data.len().try_into().unwrap_or(0);

                (data, total)
            }
            (None, false) => {
                let data: EpisodeList = query.all(&*self.db).await?.into();
                let total = data.len().try_into().unwrap_or(0);

                (data, total)
            }
        };

        Ok(ManyResponse::new(data.into(), total, page_num, page_size))
    }

    async fn create(&self, input: &CreateEpisodeInput, with_show: &bool) -> Result<Episode> {
        let episode = model::ActiveModel {
            title: Set(input.title.clone()),
            summary: Set(input.summary.clone()),
            picture: Set(input.picture.clone()),
            show_id: Set(input.show_id.clone()),
            ..Default::default()
        }
        .insert(&*self.db)
        .await?;

        let mut created: Episode = episode;

        if !with_show {
            return Ok(created);
        }

        let show = show_model::Entity::find_by_id(input.show_id.clone())
            .one(&*self.db)
            .await?;

        created.show = show;

        Ok(created)
    }
    async fn update(
        &self,
        id: &str,
        input: &UpdateEpisodeInput,
        with_show: &bool,
    ) -> Result<Episode> {
        let query = model::Entity::find_by_id(id.to_owned());

        // Pull out the `Episode` and the related `Show`, if selected
        let (episode, show) = if *with_show {
            query
                .find_also_related(show_model::Entity)
                .one(&*self.db)
                .await?
        } else {
            // If the Show isn't requested, just map to None
            query.one(&*self.db).await?.map(|p| (p, None))
        }
        .ok_or_else(|| anyhow!("Unable to find Episode with id: {}", id))?;

        let mut episode: model::ActiveModel = episode.into();

        if let Some(title) = &input.title {
            episode.title = Set(title.clone());
        }

        match &input.summary {
            Undefined => (),
            Null => episode.summary = Set(None),
            Value(value) => episode.summary = Set(Some(value.clone())),
        }

        match &input.picture {
            Undefined => (),
            Null => episode.picture = Set(None),
            Value(value) => episode.picture = Set(Some(value.clone())),
        }

        if let Some(show_id) = &input.show_id {
            episode.show_id = Set(show_id.clone());
        }

        let mut updated: Episode = episode.update(&*self.db).await?;

        // Add back the Show from above
        updated.show = show;

        Ok(updated)
    }

    async fn delete(&self, id: &str) -> Result<()> {
        let episode = model::Entity::find_by_id(id.to_owned())
            .one(&*self.db)
            .await?
            .ok_or_else(|| anyhow!("Unable to find Episode with id: {}", id))?;

        let _result = episode.delete(&*self.db).await?;

        Ok(())
    }
}

/// Provide the EpisodesService
///
/// **Provides:** `Arc<dyn EpisodesService>`
///
/// **Depends on:**
///   - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct ProvideEpisodesService {}

#[async_trait]
impl Provider for ProvideEpisodesService {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let db = i.get(&DATABASE_CONNECTION).await?;

        let service: Box<dyn EpisodesService> = Box::new(DefaultEpisodesService::new(db));

        Ok(Arc::new(service))
    }
}
