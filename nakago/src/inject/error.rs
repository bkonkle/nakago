use std::{
    fmt::{self, Debug, Display},
    sync::Arc,
};
use thiserror::Error;

use super::Key;

/// Injection Errors
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// Type ID already occupied
    Occupied(
        /// The Key of the entity type that was already provided
        Key,
    ),

    /// An instance for the given Key was not found
    NotFound {
        /// The Key of the entity that was not found
        missing: Key,

        /// The Keys that are available in the container
        available: Vec<Key>,
    },

    /// An error thrown from a Provider
    Provider(#[from] Arc<anyhow::Error>),

    /// An error thrown when an Any type cannot be downcast to the given concrete type
    TypeMismatch(
        /// The Key of the entity that was not found
        Key,
    ),

    /// An error thrown when a Key cannot be consumed and removed from the container, usually
    /// because there are still active refs to the Arc containing the dependency.
    CannotConsume(
        /// The Key of the entity that cannot be consumed
        Key,
    ),
}

/// A Dependency Injection Result
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Occupied(key) => write!(f, "{key} has already been provided"),
            Self::NotFound { missing, available } => {
                let avail_lines = if !available.is_empty() {
                    format!(
                        "\n - {}",
                        available
                            .iter()
                            .map(|k| k.to_string())
                            .collect::<Vec<String>>()
                            .join("\n\n - ")
                    )
                } else {
                    " (empty)".to_string()
                };

                write!(f, "{missing} was not found\n\nAvailable:{avail_lines}")
            }
            Self::Provider(_) => write!(f, "provider failure"),
            Self::TypeMismatch(key) => {
                write!(f, "{key} was not able to be downcast to {0}", key.type_name)
            }
            Self::CannotConsume(key) => write!(f, "{key} cannot be consumed"),
        }
    }
}
