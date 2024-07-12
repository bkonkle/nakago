#![cfg(feature = "integration")]

use anyhow::Result;
use fake::{faker::internet::en::FreeEmail, Fake};
use nakago_examples_async_graphql::domains::{
    profiles,
    role_grants::{self, model::CreateRoleGrantInput},
    users,
};
use pretty_assertions::assert_eq;
use serde_json::{json, Value};

#[cfg(test)]
mod utils;

use ulid::Ulid;
use utils::Utils;

/***
 * Query: `getCurrentUser`
 */

const GET_CURRENT_USER: &str = "
    query GetCurrentUser {
        getCurrentUser {
            id
            username
            isActive
            roles {
                roleKey
                resourceTable
                resourceId
            }
        }
    }
";

#[tokio::test]
async fn test_user_get_current_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user with this username
    let users = utils.app.get_type::<Box<dyn users::Service>>().await?;
    let user = users.create(&username).await?;

    // Create a sample RoleGrant to test the relation
    let role_grants = utils
        .app
        .get_type::<Box<dyn role_grants::Service>>()
        .await?;
    let role_grant = role_grants
        .create(&CreateRoleGrantInput {
            user_id: user.id.clone(),
            role_key: "test".to_string(),
            resource_table: "users".to_string(),
            resource_id: user.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(GET_CURRENT_USER, Value::Null, Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_user = &json["data"]["getCurrentUser"];
    let json_roles = &json_user["roles"];

    assert_eq!(status, 200);
    assert_eq!(json_user["id"], user.id);
    assert_eq!(json_user["username"], user.username);
    assert!(json_user["isActive"].as_bool().unwrap());
    assert_eq!(json_roles[0]["roleKey"], role_grant.role_key);

    Ok(())
}

#[tokio::test]
async fn test_user_get_current_no_user() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(GET_CURRENT_USER, Value::Null, Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getCurrentUser"], Value::Null);
    assert_eq!(json["errors"], Value::Null);

    Ok(())
}

/***
 * Mutation: `getOrCreateCurrentUser`
 */
const GET_OR_CREATE_CURRENT_USER: &str = "
    mutation GetOrCreateCurrentUser($input: CreateUserInput!) {
        getOrCreateCurrentUser(input: $input) {
            user {
                id
                username
                isActive
                roles {
                    roleKey
                    resourceTable
                    resourceId
                }
            }
        }
    }
";

#[tokio::test]
async fn test_user_get_or_create_current_existing() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user
    let users = utils.app.get_type::<Box<dyn users::Service>>().await?;
    let user = users.get_or_create(&username).await?;

    // Create a sample RoleGrant to test the relation
    let role_grants = utils
        .app
        .get_type::<Box<dyn role_grants::Service>>()
        .await?;
    let role_grant = role_grants
        .create(&CreateRoleGrantInput {
            user_id: user.id.clone(),
            role_key: "test".to_string(),
            resource_table: "users".to_string(),
            resource_id: user.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(
            GET_OR_CREATE_CURRENT_USER,
            json!({ "input": {}}),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_user = &json["data"]["getOrCreateCurrentUser"]["user"];
    let json_roles = &json_user["roles"];

    assert_eq!(status, 200);
    assert_eq!(json_user["id"], user.id);
    assert_eq!(json_user["username"], user.username);
    assert_eq!(json_roles[0]["roleKey"], role_grant.role_key);

    Ok(())
}

#[tokio::test]
async fn test_user_get_or_create_current_create() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(
            GET_OR_CREATE_CURRENT_USER,
            json!({ "input": {
               "profile": {
                   "email": email,
               }
            }}),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_user = &json["data"]["getOrCreateCurrentUser"]["user"];

    assert_eq!(status, 200);
    assert_eq!(json_user["username"], username);

    let user_id = json_user["id"].as_str().expect("No user id found");

    // Ensure that a related Profile was created inline
    let profiles = utils.app.get_type::<Box<dyn profiles::Service>>().await?;
    profiles
        .get_by_user_id(user_id, &false)
        .await?
        .expect("No profile id found");

    Ok(())
}

#[tokio::test]
async fn test_user_get_or_create_current_requires_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(GET_OR_CREATE_CURRENT_USER, json!({ "input": {}}), None)
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
 * Query: `updateCurrentUser`
 */
const UPDATE_CURRENT_USER: &str = "
    mutation UpdateCurrentUser($input: UpdateUserInput!) {
        updateCurrentUser(input: $input) {
            user {
                id
                username
                isActive
                roles {
                    roleKey
                    resourceTable
                    resourceId
                }
            }
        }
    }
";

#[tokio::test]
async fn test_user_update_current() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user with this username
    let users = utils.app.get_type::<Box<dyn users::Service>>().await?;
    let user = users.get_or_create(&username).await?;

    // Create a sample RoleGrant to test the relation
    let role_grants = utils
        .app
        .get_type::<Box<dyn role_grants::Service>>()
        .await?;
    let role_grant = role_grants
        .create(&CreateRoleGrantInput {
            user_id: user.id.clone(),
            role_key: "test".to_string(),
            resource_table: "users".to_string(),
            resource_id: user.id.clone(),
        })
        .await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_CURRENT_USER,
            json!({ "input": {
               "isActive": false
            }}),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_user = &json["data"]["updateCurrentUser"]["user"];
    let json_roles = &json_user["roles"];

    assert_eq!(status, 200);
    assert_eq!(json_user["username"], username);
    assert!(!json_user["isActive"].as_bool().unwrap());
    assert_eq!(json_roles[0]["roleKey"], role_grant.role_key);

    Ok(())
}

#[tokio::test]
async fn test_user_update_current_requires_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_CURRENT_USER,
            json!({ "input": {
               "isActive": false
            }}),
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

#[tokio::test]
async fn test_user_update_current_requires_user() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_CURRENT_USER,
            json!({ "input": {
               "isActive": false
            }}),
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
