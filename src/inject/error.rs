use std::fmt::{self, Debug, Display};
use thiserror::Error;

use super::Key;

/// Injection Errors
#[derive(Error, Debug)]
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
    Provider(#[from] anyhow::Error),
}

/// A Dependency Injection Result
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Occupied(name) => write!(f, "{name} has already been provided"),
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
        }
    }
}
