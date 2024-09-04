use serde::{Deserialize, Serialize};

use crate::domains::users::model::User;

/// A Session tracking details about this particular connection
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Session {
    /// A session that is not associated with a User
    #[default]
    Anonymous,

    /// A session that is associated with a User
    User {
        /// The User instance
        user: User,
    },
}

impl Session {
    /// Get the User associated with this session, if any
    #[allow(dead_code)]
    pub fn get_user(&self) -> Option<&User> {
        match self {
            Session::Anonymous => None,
            Session::User { user, .. } => Some(user),
        }
    }
}
