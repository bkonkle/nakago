#![cfg(feature = "integration")]

use anyhow::Result;
use fake::{faker::internet::en::FreeEmail, Fake, Faker};
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
 * Mutation: `createShow`
 */

const CREATE_SHOW: &str = "
    mutation CreateShow($input: CreateShowInput!) {
        createShow(input: $input) {
            show {
                id
                title
                summary
                picture
            }
        }
    }
";

/// It creates a new show
#[tokio::test]
async fn test_show_create_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user and profile with this username
    let _ = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(
            CREATE_SHOW,
            json!({
                "input": {
                    "title": "Test Show"
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_show = &json["data"]["createShow"]["show"];

    assert_eq!(status, 200);
    assert_eq!(json_show["title"], "Test Show");

    Ok(())
}

/// It requires a title
#[tokio::test]
async fn test_show_create_requires_title() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(CREATE_SHOW, json!({ "input": {}}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "title" of type "String!" is required but not provided"#
    );

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_show_create_requires_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(
            CREATE_SHOW,
            json!({
                "input": {
                    "title": "Test Show"
                }
            }),
            Some(&token),
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

/***
 * Query: `getShow`
 */

const GET_SHOW: &str = "
    query GetShow($id: ID!) {
        getShow(id: $id) {
            id
            title
            summary
            picture
        }
    }
";

/// It retrieves an existing show
#[tokio::test]
async fn test_show_get_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    let resp = utils
        .graphql
        .query(GET_SHOW, json!({ "id": show.id,}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_show = &json["data"]["getShow"];

    assert_eq!(status, 200);
    assert_eq!(json_show["id"], show.id);
    assert_eq!(json_show["title"], "Test Show");

    Ok(())
}

/// It returns nothing when no show is found
#[tokio::test]
async fn test_show_get_empty() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(GET_SHOW, json!({ "id": "dummy-id",}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getShow"], Value::Null);

    Ok(())
}

/***
 * Query: `getManyShows`
 */

const GET_MANY_SHOWS: &str = "
    query GetManyShows(
        $where: ShowCondition
        $orderBy: [ShowsOrderBy!]
        $pageSize: Int
        $page: Int
    ) {
        getManyShows(
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
            }
            count
            total
            page
            pageCount
        }
    }
";

/// It queries existing shows
#[tokio::test]
async fn test_show_get_many() -> Result<()> {
    let utils = Utils::init().await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();
    show_input.summary = Some("test-summary".to_string());

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    let mut other_show_input: CreateShowInput = Faker.fake();
    other_show_input.title = "Test Show 2".to_string();
    other_show_input.summary = Some("test-summary-2".to_string());

    let other_show = shows.create(&other_show_input).await?;

    let resp = utils
        .graphql
        .query(
            GET_MANY_SHOWS,
            json!({
                "where": {
                    "idsIn": vec![show.id.clone(), other_show.id.clone()],
                },
            }),
            None,
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_result = &json["data"]["getManyShows"];
    let json_show = &json_result["data"][0];
    let json_other_show = &json_result["data"][1];

    assert_eq!(status, 200);

    assert_eq!(json_result["count"], 2);
    assert_eq!(json_result["total"], 2);
    assert_eq!(json_result["page"], 1);
    assert_eq!(json_result["pageCount"], 1);

    assert_eq!(json_show["id"], show.id);
    assert_eq!(json_show["title"], "Test Show");
    assert_eq!(json_show["summary"], show.summary.unwrap());

    assert_eq!(json_other_show["id"], other_show.id);
    assert_eq!(json_other_show["title"], "Test Show 2");
    assert_eq!(json_other_show["summary"], other_show.summary.unwrap());

    Ok(())
}

/***
 * Mutation: `updateShow`
 */

const UPDATE_SHOW: &str = "
    mutation UpdateShow($id: ID!, $input: UpdateShowInput!) {
        updateShow(id: $id, input: $input) {
            show {
                id
                title
                summary
                picture
            }
        }
    }
";

/// It updates an existing show
#[tokio::test]
async fn test_show_update_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a User
    let users = utils.app.get::<Box<dyn users::Service>>().await?;
    let user = users.create(&username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    // Grant the admin role to this User for this Show
    let role_grants = utils.app.get::<Box<dyn role_grants::Service>>().await?;
    role_grants
        .create(&CreateRoleGrantInput {
            role_key: "admin".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_SHOW,
            json!({
                "id": show.id,
                "input": {
                    "summary": "Something else"
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_show = &json["data"]["updateShow"]["show"];

    assert_eq!(status, 200);

    assert_eq!(json_show["id"], show.id);
    assert_eq!(json_show["title"], "Test Show");
    assert_eq!(json_show["summary"], "Something else");

    Ok(())
}

/// It returns an error if no existing show is found
#[tokio::test]
async fn test_show_update_not_found() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_SHOW,
            json!({
                "id": "test-id",
                "input": {
                    "summary": "Something else"
                }
            }),
            None,
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unable to find existing Show");
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_show_update_requires_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_SHOW,
            json!({
                "id": show.id,
                "input": {
                    "summary": "Something else"
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
async fn test_show_update_requires_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a User
    let users = utils.app.get::<Box<dyn users::Service>>().await?;
    let _ = users.create(&username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_SHOW,
            json!({
                "id": show.id,
                "input": {
                    "summary": "Something else"
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
 * Mutation: `deleteShow`
 */

const DELETE_SHOW: &str = "
    mutation DeleteShow($id: ID!) {
        deleteShow(id: $id)
    }
";

/// It deletes an existing show
#[tokio::test]
async fn test_show_delete_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a User
    let users = utils.app.get::<Box<dyn users::Service>>().await?;
    let user = users.create(&username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    // Grant the admin role to this User for this Show
    let role_grants = utils.app.get::<Box<dyn role_grants::Service>>().await?;
    role_grants
        .create(&CreateRoleGrantInput {
            role_key: "admin".to_string(),
            user_id: user.id.clone(),
            resource_table: "shows".to_string(),
            resource_id: show.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(DELETE_SHOW, json!({"id": show.id}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert!(json["data"]["deleteShow"].as_bool().unwrap());

    Ok(())
}

/// It returns an error if no existing show is found
#[tokio::test]
async fn test_show_delete_not_found() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(DELETE_SHOW, json!({"id": "test-id"}), None)
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unable to find existing Show");
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_show_delete_requires_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    let resp = utils
        .graphql
        .query(DELETE_SHOW, json!({"id": show.id}), None)
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
async fn test_show_delete_requires_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a User
    let users = utils.app.get::<Box<dyn users::Service>>().await?;
    let _ = users.create(&username).await?;

    let mut show_input: CreateShowInput = Faker.fake();
    show_input.title = "Test Show".to_string();

    let shows = utils.app.get::<Box<dyn shows::Service>>().await?;
    let show = shows.create(&show_input).await?;

    let resp = utils
        .graphql
        .query(DELETE_SHOW, json!({"id": show.id}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Forbidden");
    assert_eq!(json["errors"][0]["extensions"]["code"], 403);

    Ok(())
}
