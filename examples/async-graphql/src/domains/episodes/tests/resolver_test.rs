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
        schema::{self, test::SCHEMA},
        service::{MockService, SERVICE},
    },
    shows,
};

async fn setup(service: MockService) -> Result<Inject> {
    let i = Inject::default();

    i.inject(&SERVICE, Box::new(service)).await?;

    i.provide(
        &shows::SERVICE,
        shows::service::test::ProvideMock::default(),
    )
    .await?;

    i.provide(&shows::LOADER, shows::loaders::Provide::default())
        .await?;

    i.provide(&SCHEMA, schema::test::Provide::default()).await?;

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

    let mut service = MockService::default();
    service
        .expect_get()
        .with(eq(episode_id), eq(&true))
        .times(1)
        .returning(move |_, _| Ok(Some(episode.clone())));

    let i = setup(service).await?;

    let schema = i.get(&SCHEMA).await?;

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
