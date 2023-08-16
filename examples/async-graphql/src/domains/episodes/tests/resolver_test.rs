use anyhow::Result;
use async_graphql::{Request, Variables};
use fake::{Fake, Faker};
use mockall::predicate::*;
use nakago::Inject;
use pretty_assertions::assert_eq;
use serde_json::json;

use crate::domains::{
    episodes::{
        model::Episode,
        schema::{ProvideEpisodesSchema, EPISODES_SCHEMA},
        service::{MockEpisodesService, EPISODES_SERVICE},
    },
    shows::service::SHOWS_SERVICE,
    shows::{
        loaders::{ProvideShowLoader, SHOW_LOADER},
        service::test::ProvideMockShowsService,
    },
};

async fn setup(service: MockEpisodesService) -> Result<Inject> {
    let i = Inject::default();

    i.inject(&EPISODES_SERVICE, Box::new(service)).await?;

    i.provide(&SHOWS_SERVICE, ProvideMockShowsService::default())
        .await?;

    i.provide(&SHOW_LOADER, ProvideShowLoader::default())
        .await?;

    i.provide(&EPISODES_SCHEMA, ProvideEpisodesSchema::default())
        .await?;

    Ok(i)
}

/***
 * Query: `getEpisode`
 */

const GET_EPISODE: &str = "
    query GetEpisode($id: ID!) {
        getEpisode(id: $id) {
            id
            title
            summary
            picture
            show {
                id
            }
        }
    }
";

#[tokio::test]
async fn test_episodes_resolver_get_simple() -> Result<()> {
    let episode_id = "Test Episode";
    let episode_title = "Test Episode 1";

    let mut episode: Episode = Faker.fake();
    episode.id = episode_id.to_string();
    episode.title = episode_title.to_string();
    episode.show = Some(Faker.fake());

    let mut service = MockEpisodesService::default();
    service
        .expect_get()
        .with(eq(episode_id), eq(&true))
        .times(1)
        .returning(move |_, _| Ok(Some(episode.clone())));

    let i = setup(service).await?;

    let schema = i.get(&EPISODES_SCHEMA).await?;

    let result = schema
        .execute(
            Request::new(GET_EPISODE).variables(Variables::from_json(json!({ "id": episode_id }))),
        )
        .await;

    let data = result.data.into_json()?;
    let json_episode = &data["getEpisode"];

    assert_eq!(json_episode["id"], episode_id);
    assert_eq!(json_episode["title"], episode_title);

    Ok(())
}
