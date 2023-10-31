#![cfg(feature = "integration")]

use anyhow::Result;

#[cfg(test)]
mod utils;

use hyper::{body::to_bytes, Method};
use serde_json::Value;
use ulid::Ulid;
use utils::TestUtils;

#[tokio::test]
async fn test_get_username_success() -> Result<()> {
    let utils = TestUtils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let req = utils
        .http
        .call(Method::GET, "/username", Value::Null, Some(&token))?;

    let resp = utils.http_client.request(req).await?;

    let status = resp.status();
    let body = to_bytes(resp.into_body()).await?;

    let json: Value = serde_json::from_slice(&body)?;

    assert_eq!(status, 200);
    assert_eq!(json["username"], username);

    Ok(())
}
