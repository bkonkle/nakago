use std::sync::Arc;

use axum::{extract::State, Json};
use nakago_axum::auth::Subject;
use serde::{Deserialize, Serialize};

use crate::users::service::UsersService;

// Health
// ------

/// A Health Check Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// The Status code
    code: usize,

    /// Whether the check was successful or not
    success: bool,
}

/// Handle health check requests
pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        code: 200,
        success: true,
    })
}

// Users
// -----

/// State for the User Handler
#[derive(Clone)]
pub struct UserState {
    users: Arc<Box<dyn UsersService>>,
}

impl UserState {
    /// Create a new GraphQLState instance
    pub fn new(users: Arc<Box<dyn UsersService>>) -> Self {
        Self { users }
    }
}

/// A User Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    /// The User's current Display Name
    display_name: String,
}

/// Handle Get User Requests
pub async fn get_user_handler(State(state): State<UserState>, sub: Subject) -> Json<UserResponse> {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        state.users.get_by_username(username).await.unwrap_or(None)
    } else {
        None
    };

    Json(UserResponse {
        display_name: user.map_or_else(|| "Anonymous".to_owned(), |u| u.display_name),
    })
}

/// Handle Update User Requests
pub async fn update_user_handler(
    State(state): State<UserState>,
    sub: Subject,
) -> Json<UserResponse> {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        state.users.get_by_username(username).await.unwrap_or(None)
    } else {
        None
    };

    Json(UserResponse {
        display_name: user.map_or_else(|| "Anonymous".to_owned(), |u| u.display_name),
    })
}
