#![cfg(feature = "integration")]

use anyhow::Result;
use fake::{faker::internet::en::FreeEmail, Fake};
use nakago_examples_async_graphql::domains::users;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use ulid::Ulid;

#[cfg(test)]
mod utils;

use utils::Utils;

/***
 * Mutation: `createProfile`
 */

const CREATE_PROFILE: &str = "
    mutation CreateProfile($input: CreateProfileInput!) {
        createProfile(input: $input) {
            profile {
                id
                email
                displayName
                picture
                user {
                    id
                }
            }
        }
    }
";

/// It creates a new user profile
#[tokio::test]
async fn test_profile_create_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user and profile with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let user = users.create(&username).await?;

    let resp = utils
        .graphql
        .query(
            CREATE_PROFILE,
            json!({
                "input": {
                    "email": email,
                    "userId": user.id.clone(),
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_profile = &json["data"]["createProfile"]["profile"];
    let json_user = &json_profile["user"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["email"], email.clone());
    assert_eq!(json_user["id"], user.id.clone());

    Ok(())
}

/// It requires an email address and a userId
#[tokio::test]
async fn test_profile_create_requires_email_user_id() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .graphql
        .query(CREATE_PROFILE, json!({ "input": {}}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        r#"Invalid value for argument "input", field "email" of type "String!" is required but not provided"#
    );

    // Now provide the "email" and try again
    let resp = utils
        .graphql
        .query(
            CREATE_PROFILE,
            json!({
                "input": {
                    "email": email,
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
        r#"Invalid value for argument "input", field "userId" of type "String!" is required but not provided"#
    );

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_profile_create_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(
            CREATE_PROFILE,
            json!({
                "input": {
                    "email": "dummy-email",
                    "userId": "dummy-user-id"
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
async fn test_profile_create_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user and profile with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let _ = users.create(&username).await?;

    let resp = utils
        .graphql
        .query(
            CREATE_PROFILE,
            json!({
                "input": {
                    "email": email,
                    "userId": "dummy-user-id",
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
 * Query: `getProfile`
 */

const GET_PROFILE: &str = "
    query GetProfile($id: ID!) {
        getProfile(id: $id) {
            id
            email
            displayName
            picture
            user {
                id
            }
        }
    }
";

/// It retrieves an existing user profile
#[tokio::test]
async fn test_profile_get_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(GET_PROFILE, json!({ "id": profile.id,}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_profile = &json["data"]["getProfile"];
    let json_user = &json_profile["user"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], email.clone());
    assert_eq!(json_user["id"], user.id);

    Ok(())
}

/// It returns nothing when no profile is found
#[tokio::test]
async fn test_profile_get_empty() -> Result<()> {
    let utils = Utils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user with this username
    let users = utils.i.get::<Box<dyn users::Service>>().await?;
    let _ = users.create(&username).await?;

    let resp = utils
        .graphql
        .query(GET_PROFILE, json!({ "id": "dummy-id",}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["data"]["getProfile"], Value::Null);

    Ok(())
}

/// It censors responses for anonymous users
#[tokio::test]
async fn test_profile_get_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();

    // Create a user and profile with this username
    let (_, profile) = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(GET_PROFILE, json!({ "id": profile.id,}), None)
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_profile = &json["data"]["getProfile"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], Value::Null);
    assert_eq!(json_profile["user"], Value::Null);

    Ok(())
}

/// It censors responses for unauthorized users
#[tokio::test]
async fn test_profile_get_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let dummy_username = Ulid::new().to_string();

    // Create a user with a different username
    let (_, profile) = utils
        .create_user_and_profile(&dummy_username, &email)
        .await?;

    let resp = utils
        .graphql
        .query(GET_PROFILE, json!({ "id": profile.id,}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_profile = &json["data"]["getProfile"];

    assert_eq!(status, 200);
    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], Value::Null);
    assert_eq!(json_profile["user"], Value::Null);

    Ok(())
}

/***
 * Query: `getManyProfiles`
 */

const GET_MANY_PROFILES: &str = "
    query GetManyProfiles(
        $where: ProfileCondition
        $orderBy: [ProfilesOrderBy!]
        $pageSize: Int
        $page: Int
    ) {
        getManyProfiles(
            where: $where
            orderBy: $orderBy
            pageSize: $pageSize
            page: $page
        ) {
            data {
                id
                email
                displayName
                picture
                user {
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

/// It queries existing profiles and censors responses for unauthorized users
#[tokio::test]
async fn test_profile_get_many_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let dummy_username = Ulid::new().to_string();

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(&username, &email).await?;

    // Create a user with another username
    let (_, other_profile) = utils
        .create_user_and_profile(&dummy_username, "other@email.address")
        .await?;

    let resp = utils
        .graphql
        .query(
            GET_MANY_PROFILES,
            json!({
                "where": {
                    "idsIn": vec![profile.id.clone(), other_profile.id.clone()],
                },
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_profile = &json["data"]["getManyProfiles"]["data"][0];
    let json_user = &json_profile["user"];

    let json_other_profile = &json["data"]["getManyProfiles"]["data"][1];

    assert_eq!(status, 200);

    assert_eq!(json["data"]["getManyProfiles"]["count"], 2);
    assert_eq!(json["data"]["getManyProfiles"]["total"], 2);
    assert_eq!(json["data"]["getManyProfiles"]["page"], 1);
    assert_eq!(json["data"]["getManyProfiles"]["pageCount"], 1);

    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], email.clone());
    assert_eq!(json_user["id"], user.id);

    assert_eq!(json_other_profile["id"], other_profile.id);
    assert_eq!(json_other_profile["email"], Value::Null); // Because of censoring
    assert_eq!(json_other_profile["user"], Value::Null); // Because of censoring

    Ok(())
}

/// It censors responses for anonymous users
#[tokio::test]
async fn test_profile_get_many_anon() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();

    // Create a user and profile with this username
    let (_, profile) = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(
            GET_MANY_PROFILES,
            json!({
                "where": {
                    "idsIn": vec![profile.id.clone()],
                },
            }),
            None,
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_profile = &json["data"]["getManyProfiles"]["data"][0];

    assert_eq!(status, 200);

    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], Value::Null);
    assert_eq!(json_profile["user"], Value::Null);

    Ok(())
}

/***
 * Mutation: `updateProfile`
 */

const UPDATE_PROFILE: &str = "
    mutation UpdateProfile($id: ID!, $input: UpdateProfileInput!) {
        updateProfile(id: $id, input: $input) {
            profile {
                id
                email
                displayName
                picture
                user {
                    id
                }
            }
        }
    }
";

/// It updates an existing user profile
#[tokio::test]
async fn test_profile_update_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user and profile with this username
    let (user, profile) = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_PROFILE,
            json!({
                "id": profile.id,
                "input": {
                    "displayName": "Test Name"
                }
            }),
            Some(&token),
        )
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    let json_profile = &json["data"]["updateProfile"]["profile"];
    let json_user = &json_profile["user"];

    assert_eq!(status, 200);

    assert_eq!(json_profile["id"], profile.id);
    assert_eq!(json_profile["email"], email.clone());
    assert_eq!(json_profile["displayName"], "Test Name");
    assert_eq!(json_user["id"], user.id);

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_profile_update_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();

    // Create a user and profile with this username
    let (_, profile) = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_PROFILE,
            json!({
                "id": profile.id,
                "input": {
                    "displayName": "Test Name"
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

/// It returns an error if no existing profile was found
#[tokio::test]
async fn test_profile_update_not_found() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_PROFILE,
            json!({
                "id": "test-id",
                "input": {
                    "displayName": "Test Name"
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
        "Unable to find existing Profile"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authorization
#[tokio::test]
async fn test_profile_update_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let dummy_username = Ulid::new().to_string();

    // Create a dummy user and profile
    let (_, profile) = utils
        .create_user_and_profile(&dummy_username, "other@email.address")
        .await?;

    // Create a user and profile for the Alt user
    let _ = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(
            UPDATE_PROFILE,
            json!({
                "id": profile.id,
                "input": {
                    "displayName": "Test Name"
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
 * Mutation: `deleteProfile`
 */

const DELETE_PROFILE: &str = "
    mutation DeleteProfile($id: ID!) {
        deleteProfile(id: $id)
    }
";

/// It deletes an existing user profile
#[tokio::test]
async fn test_profile_delete_simple() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    // Create a user and profile with this username
    let (_, profile) = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(DELETE_PROFILE, json!({"id": profile.id}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert!(json["data"]["deleteProfile"].as_bool().unwrap());

    Ok(())
}

/// It requires authentication
#[tokio::test]
async fn test_profile_delete_authn() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();

    // Create a user and profile with this username
    let (_, profile) = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(DELETE_PROFILE, json!({"id": profile.id}), None)
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Unauthorized");
    assert_eq!(json["errors"][0]["extensions"]["code"], 401);

    Ok(())
}

/// It returns an error if no existing profile was found
#[tokio::test]
async fn test_profile_delete_not_found() -> Result<()> {
    let utils = Utils::init().await?;

    let resp = utils
        .graphql
        .query(DELETE_PROFILE, json!({"id": "test-id"}), None)
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(
        json["errors"][0]["message"],
        "Unable to find existing Profile"
    );
    assert_eq!(json["errors"][0]["extensions"]["code"], 404);

    Ok(())
}

/// It requires authorization
#[tokio::test]
async fn test_profile_delete_authz() -> Result<()> {
    let utils = Utils::init().await?;

    let email: String = FreeEmail().fake();
    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let dummy_username = Ulid::new().to_string();

    // Create a dummy user and profile
    let (_, profile) = utils
        .create_user_and_profile(&dummy_username, "other@email.address")
        .await?;

    // Create a user and profile for the Alt user
    let _ = utils.create_user_and_profile(&username, &email).await?;

    let resp = utils
        .graphql
        .query(DELETE_PROFILE, json!({"id": profile.id}), Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["errors"][0]["message"], "Forbidden");
    assert_eq!(json["errors"][0]["extensions"]["code"], 403);

    Ok(())
}
