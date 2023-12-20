#![cfg(feature = "integration")]

use anyhow::Result;

#[cfg(test)]
mod utils;

use serde_json::Value;
use ulid::Ulid;
use utils::TestUtils;

#[tokio::test]
async fn test_get_username_success() -> Result<()> {
    let utils = TestUtils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .http
        .get_json("/username", Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["username"], username);

    Ok(())
}
