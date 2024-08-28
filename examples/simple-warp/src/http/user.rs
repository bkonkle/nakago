use std::convert::Infallible;

use nakago::Inject;
use nakago_warp::auth::Subject;
use serde_derive::{Deserialize, Serialize};
use warp::reply::Reply;

/// A Username Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameResponse {
    /// The Status code
    code: usize,

    /// The username, or "(anonymous)"
    username: String,
}

impl Reply for UsernameResponse {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status(warp::reply::json(&self), warp::http::StatusCode::OK)
            .into_response()
    }
}

/// Handle Get Username requests
pub async fn handle_get_username(_: Inject, sub: Subject) -> Result<UsernameResponse, Infallible> {
    let username = if let Subject(Some(username)) = sub {
        username.clone()
    } else {
        "(anonymous)".to_string()
    };

    Ok(UsernameResponse {
        code: 200,
        username,
    })
}
