use axum::Json;
use nakago_axum::auth::Subject;
use serde_derive::{Deserialize, Serialize};

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
