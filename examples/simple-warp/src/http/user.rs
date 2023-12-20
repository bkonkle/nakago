use std::convert::Infallible;

use nakago::Inject;
use nakago_warp::{
    auth::{subject::with_auth, Subject},
    Route,
};
use serde_derive::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Filter, Reply};

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

/// Create a Get Username Route
pub fn get_username(filter: BoxedFilter<(Inject,)>) -> Route {
    warp::path("username")
        .and(warp::get())
        .and(with_auth(filter))
        .and_then(handle_get_username)
        .map(|a| Box::new(a) as Box<dyn Reply>)
        .boxed()
}

/// Handle Get Username requests
async fn handle_get_username(_: Inject, sub: Subject) -> Result<UsernameResponse, Infallible> {
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
