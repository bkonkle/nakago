#![cfg(feature = "integration")]

use anyhow::Result;
use fake::{Fake, Faker};
use nakago_examples_async_graphql::domains::{
    role_grants::{self, model::CreateRoleGrantInput},
    shows::{self, mutation::CreateShowInput},
    users,
};
use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use ulid::Ulid;

#[cfg(test)]
mod utils;

use utils::Utils;

/***
 * Mutation: `createEpisode`
 */
const CREATE_EPISODE: &str = "
    mutation CreateEpisode($input: CreateEpisodeInput!) {
        createEpisode(input: $input) {
            episode {
                id
                title
                summary
                picture
                show {
                    id
                }
            }
        }
    }
";

/// It creates a new episode
#[tokio::test]
async fn test_episode_create_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user and a show
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let user = users.create(&username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.i.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    // Grant the manager role to this user for this episode's show
    let role_grants = utils.i.get::<Box<dyn role_grants::Service>>().await?;
    role_grants
        .create(&CreateRoleGrantInput {
            role_key: "manager".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(
            CREATE_EPISODE,
            json!({
                "input": {
                    "title": "Test Episode 1",
                    "showId": show.id.clone(),
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();
    if status != 200 {
        panic!("HTTP Response was not OK: {}", resp.text().await?);
    }

    let json = resp.json::<Value>().await?;

    let json_episode = &json["data"]["createEpisode"]["episode"];
    let json_show = &json_episode["show"];

    assert_eq!(json_episode["title"], "Test Episode 1");
    assert_eq!(json_show["id"], show.id.clone());

    Ok(())
}

/// It requires a title and a showId
#[tokio::test]
async fn test_episode_create_requires_title_show_id() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(CREATE_EPISODE, json!({ "input": {}}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "title" of type "String!" is required but not provided"#
    );

    // Now provide the "email" and try again
    let resp = utils
        .graphql
        .query(
            CREATE_EPISODE,
            json!({
                "input": {
                    "title": "Test Episode 1",
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "showId" of type "String!" is required but not provided"#
    );

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_episode_create_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.i.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    let resp = utils
        .graphql
        .query(
            CREATE_EPISODE,
            json!({
                "input": {
                    "title": "dummy-title",
                    "showId": show.id
                }
            }),
            None,
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unauthorized");
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/// It requires authorization
#[tokio::test]
async fn test_episode_create_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.i.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    // Create a user with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let _ = users.create(&username).await?;

    let resp = utils
        .graphql
        .query(
            CREATE_EPISODE,
            json!({
                "input": {
                    "title": "Test Episode 1",
                    "showId": show.id,
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Forbidden");
    assert_eq!(json["errors"][0]["extensions"]["code"], 403);

    Ok(())
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

/// It retrieves an existing episode
#[tokio::test]
async fn test_episode_get() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let resp = utils
        .graphql
        .query(GET_EPISODE, json!({ "id": episode.id,}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_episode = &json["data"]["getEpisode"];
    let json_show = &json_episode["show"];

    assert_eq!(status, 200);
    assert_eq!(json_episode["id"], episode.id);
    assert_eq!(json_episode["title"], "Test Episode 1");
    assert_eq!(json_show["id"], show.id);

    Ok(())
}

/// It returns nothing when no episode is found
#[tokio::test]
async fn test_episode_get_empty() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(GET_EPISODE, json!({ "id": "dummy-id",}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getEpisode"], Value::Null);

    Ok(())
}

/***
 * Query: `getManyEpisodes`
 */
const GET_MANY_EPISODES: &str = "
    query GetManyEpisodes(
        $where: EpisodeCondition
        $orderBy: [EpisodesOrderBy!]
        $pageSize: Int
        $page: Int
    ) {
        getManyEpisodes(
            where: $where
            orderBy: $orderBy
            pageSize: $pageSize
            page: $page
        ) {
            data {
                id
                title
                summary
                picture
                show {
                    id
                }
            }
            count
            total
            page
            pageCount
        }
    }
";

/// It queries existing episodes
#[tokio::test]
async fn test_episode_get_many() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let (other_show, other_episode) = utils
        .create_show_and_episode("Test Show 2", "Test Episode 1")
        .await?;

    let resp = utils
        .graphql
        .query(
            GET_MANY_EPISODES,
            json!({
                "where": {
                    "idsIn": vec![episode.id.clone(), other_episode.id.clone()],
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_episode = &json["data"]["getManyEpisodes"]["data"][0];
    let json_show = &json_episode["show"];

    let json_other_episode = &json["data"]["getManyEpisodes"]["data"][1];
    let json_other_show = &json_other_episode["show"];

    assert_eq!(status, 200);

    assert_eq!(json["data"]["getManyEpisodes"]["count"], 2);
    assert_eq!(json["data"]["getManyEpisodes"]["total"], 2);
    assert_eq!(json["data"]["getManyEpisodes"]["page"], 1);
    assert_eq!(json["data"]["getManyEpisodes"]["pageCount"], 1);

    assert_eq!(json_episode["id"], episode.id);
    assert_eq!(json_episode["title"], "Test Episode 1");
    assert_eq!(json_show["id"], show.id);

    assert_eq!(json_other_episode["id"], other_episode.id);
    assert_eq!(json_other_episode["title"], "Test Episode 1");
    assert_eq!(json_other_show["id"], other_show.id);

    Ok(())
}

/***
 * Mutation: `updateEpisode`
 */
const UPDATE_EPISODE: &str = "
    mutation UpdateEpisode($id: ID!, $input: UpdateEpisodeInput!) {
        updateEpisode(id: $id, input: $input) {
            episode {
                id
                title
                summary
                picture
                show {
                    id
                }
            }
        }
    }
";

/// It updates an existing episode
#[tokio::test]
async fn test_episode_update() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let user = users.create(&username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    // Grant the manager role to this user for this episode's show
    let role_grants = utils.i.get::<Box<dyn role_grants::Service>>().await?;
    role_grants
        .create(&CreateRoleGrantInput {
            role_key: "manager".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_EPISODE,
            json!({
                "id": episode.id,
                "input": {
                    "summary": "Test Summary"
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_episode = &json["data"]["updateEpisode"]["episode"];
    let json_show = &json_episode["show"];

    assert_eq!(status, 200);

    assert_eq!(json_episode["id"], episode.id);
    assert_eq!(json_episode["title"], "Test Episode 1");
    assert_eq!(json_episode["summary"], "Test Summary");
    assert_eq!(json_show["id"], show.id);

    Ok(())
}

/// It returns an error if no existing episode was found
#[tokio::test]
async fn test_episode_update_not_found() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_EPISODE,
            json!({
                "id": "test-id",
                "input": {
                    "summary": "Test Summary"
                }
            }),
            None,
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "Unable to find existing Episode"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_episode_update_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let (_, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_EPISODE,
            json!({
                "id": episode.id,
                "input": {
                    "summary": "Test Summary"
                }
            }),
            None,
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unauthorized");
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/// It requires authorization
#[tokio::test]
async fn test_episode_update_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let _ = users.create(&username).await?;

    let (_, episode) = utils
        .create_show_and_episode("Test Show 2", "Test Episode 1")
        .await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_EPISODE,
            json!({
                "id": episode.id,
                "input": {
                    "summary": "Test Summary"
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Forbidden");
    assert_eq!(json["errors"][0]["extensions"]["code"], 403);

    Ok(())
}

/***
 * Mutation: `deleteEpisode`
 */
const DELETE_EPISODE: &str = "
    mutation DeleteEpisode($id: ID!) {
        deleteEpisode(id: $id)
    }
";

/// It deletes an existing user episode
#[tokio::test]
async fn test_episode_delete() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let user = users.create(&username).await?;

    let (show, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    // Grant the manager role to this user for this episode's show
    let role_grants = utils.i.get::<Box<dyn role_grants::Service>>().await?;
    role_grants
        .create(&CreateRoleGrantInput {
            role_key: "manager".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(DELETE_EPISODE, json!({"id": episode.id}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert!(json["data"]["deleteEpisode"].as_bool().unwrap());

    Ok(())
}

/// It returns an error if no existing episode was found
#[tokio::test]
async fn test_episode_delete_not_found() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(DELETE_EPISODE, json!({"id": "test-id"}), None)
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "Unable to find existing Episode"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_episode_delete_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let (_, episode) = utils
        .create_show_and_episode("Test Show", "Test Episode 1")
        .await?;

    let resp = utils
        .graphql
        .query(DELETE_EPISODE, json!({"id": episode.id}), None)
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unauthorized");
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/// It requires authorization
#[tokio::test]
async fn test_episode_delete_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let _ = users.create(&username).await?;

    let (_, episode) = utils
        .create_show_and_episode("Test Show 2", "Test Episode 1")
        .await?;

    let resp = utils
        .graphql
        .query(DELETE_EPISODE, json!({"id": episode.id}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Forbidden");
    assert_eq!(json["errors"][0]["extensions"]["code"], 403);

    Ok(())
}
