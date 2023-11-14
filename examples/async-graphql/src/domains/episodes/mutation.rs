use std::sync::Arc;

use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, InputObject, MaybeUndefined, Object, Result,
    SimpleObject,
};
use async_trait::async_trait;
use derive_new::new;
use fake::{Dummy, Fake, Faker};
use hyper::StatusCode;
use nakago::{inject, Inject, Provider, Tag};
use nakago_async_graphql::utils::{as_graphql_error, dummy_maybe_undef, graphql_error};
use nakago_derive::Provider;
use oso::Oso;
use rand::Rng;

use crate::domains::{shows, shows::model::Show, users::model::User};

use super::{model::Episode, Service, SERVICE};

/// Tag(episodes::Mutation)
pub const MUTATION: Tag<EpisodesMutation> = Tag::new("episodes::Mutation");

/// The `CreateEpisodeInput` input type
#[derive(Clone, Default, Dummy, Eq, PartialEq, InputObject)]
pub struct CreateEpisodeInput {
    /// The Episode's title
    pub title: String,

    /// The Episode's description summary
    pub summary: Option<String>,

    /// The Episode's picture
    pub picture: Option<String>,

    /// The Episode's Show id
    pub show_id: String,
}

/// The `UpdateEpisodeInput` input type
#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct UpdateEpisodeInput {
    /// The Episode's title
    pub title: Option<String>,

    /// The Episode's description summary
    pub summary: MaybeUndefined<String>,

    /// The Episode's picture
    pub picture: MaybeUndefined<String>,

    /// The Episode's Show id
    pub show_id: Option<String>,
}

impl Dummy<Faker> for UpdateEpisodeInput {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        UpdateEpisodeInput {
            title: Faker.fake(),
            summary: dummy_maybe_undef(config, rng),
            picture: dummy_maybe_undef(config, rng),
            show_id: Faker.fake(),
        }
    }
}

/// The `MutateEpisodeResult` type
#[derive(Clone, Default, Dummy, Eq, PartialEq, SimpleObject)]
pub struct MutateEpisodeResult {
    /// The Episode's subscriber id
    pub episode: Option<Episode>,
}

/// The Mutation segment for Episodes
#[derive(new)]
pub struct EpisodesMutation {
    service: Arc<Box<dyn Service>>,
    shows: Arc<Box<dyn shows::Service>>,
}

/// Mutations for the Episode model
#[Object]
impl EpisodesMutation {
    /// Create a new Episode
    pub async fn create_episode(
        &self,
        ctx: &Context<'_>,
        input: CreateEpisodeInput,
    ) -> Result<MutateEpisodeResult> {
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the related Show for authorization
        let show = self
            .shows
            .get(&input.show_id)
            .await
            .map_err(as_graphql_error(
                "Unable while retrieving Show",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "manage_episodes", show.clone())? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        let episode = self
            .service
            .create(&input, &false)
            .await
            .map_err(as_graphql_error(
                "Error while creating Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateEpisodeResult {
            episode: Some(Episode {
                show: Some(show),
                ..episode
            }),
        })
    }

    /// Update an existing Episode
    pub async fn update_episode(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UpdateEpisodeInput,
    ) -> Result<MutateEpisodeResult> {
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the existing Episode for authorization
        let existing = self
            .service
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Episode", StatusCode::NOT_FOUND)
            })?;

        let show = existing
            .show
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "manage_episodes", show.clone())? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        // Check to see if the associated User is selected
        let with_show = ctx.look_ahead().field("episode").field("show").exists();

        // Use the already retrieved Episode to update the record
        let episode = self
            .service
            .update(&existing.id, &input, &with_show)
            .await
            .map_err(as_graphql_error(
                "Error while updating Profile",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?;

        Ok(MutateEpisodeResult {
            episode: Some(episode),
        })
    }

    /// Remove an existing Episode
    pub async fn delete_episode(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
        let user = ctx.data_unchecked::<Option<User>>();
        let oso = ctx.data_unchecked::<Oso>();

        // Retrieve the related Show for authorization
        let episode = self
            .service
            .get(&id, &true)
            .await
            .map_err(as_graphql_error(
                "Error while fetching Episode",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))?
            .ok_or_else(|| {
                graphql_error("Unable to find existing Episode", StatusCode::NOT_FOUND)
            })?;

        let show = episode
            .show
            .ok_or_else(|| graphql_error("Unable to find existing Show", StatusCode::NOT_FOUND))?;

        // Check authentication and authorization
        if let Some(user) = user {
            if !oso.is_allowed(user.clone(), "manage_episodes", show.clone())? {
                return Err(graphql_error("Forbidden", StatusCode::FORBIDDEN));
            }
        } else {
            return Err(graphql_error("Unauthorized", StatusCode::UNAUTHORIZED));
        }

        self.service.delete(&id).await.map_err(as_graphql_error(
            "Error while deleting Episode",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))?;

        Ok(true)
    }
}

#[ComplexObject]
impl Episode {
    #[graphql(name = "show")]
    async fn resolve_show(&self, ctx: &Context<'_>) -> Result<Option<Show>> {
        if let Some(show) = self.show.clone() {
            return Ok(Some(show));
        }

        let loader = ctx.data_unchecked::<DataLoader<shows::Loader>>();
        let show = loader.load_one(self.show_id.clone()).await?;

        Ok(show)
    }
}

/// Provide the EpisodesMutation
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<EpisodesMutation> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<EpisodesMutation>> {
        let service = i.get(&SERVICE).await?;
        let shows = i.get(&shows::SERVICE).await?;

        Ok(Arc::new(EpisodesMutation::new(service, shows)))
    }
}
