use std::sync::Arc;

use async_trait::async_trait;
use axum::Json;
use nakago::{inject, Inject, Provider, Tag};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;
use serde_derive::{Deserialize, Serialize};

/// Tag(user::Controller)
pub const CONTROLLER: Tag<Controller> = Tag::new("user::Controller");

/// A Username Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameResponse {
    /// The Status code
    code: usize,

    /// The username, or "(anonymous)"
    username: String,
}

#[derive(Clone)]
pub struct Controller {}

impl Controller {
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
}

#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Controller> for Provide {
    async fn provide(self: Arc<Self>, _: Inject) -> inject::Result<Arc<Controller>> {
        Ok(Arc::new(Controller {}))
    }
}
