use std::sync::Arc;

use async_trait::async_trait;
use axum::{routing::get, Json, Router};
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::{auth::Subject, Route};
use nakago_derive::Provider;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::Mutex;

/// Tag(user::get_username::Route)
pub const GET_USERNAME_ROUTE: Tag<Route> = Tag::new("user::get_username::Route");

/// A Username Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameResponse {
    /// The Status code
    code: usize,

    /// The username, or "(anonymous)"
    username: String,
}

/// Handle Get Username requests
pub async fn get_username(sub: Subject) -> Json<UsernameResponse> {
    let username = if let Subject(Some(username)) = sub {
        username.clone()
    } else {
        "(anonymous)".to_string()
    };

    Json(UsernameResponse {
        code: 200,
        username,
    })
}

/// A Provider for the Get Username route
#[derive(Default)]
pub struct ProvideGetUsername {}

#[Provider]
#[async_trait]
impl Provider<Route> for ProvideGetUsername {
    async fn provide(self: Arc<Self>, _: Inject) -> inject::Result<Arc<Route>> {
        let route = Router::new().route("/username", get(get_username));

        Ok(Arc::new(Mutex::new(route)))
    }
}
