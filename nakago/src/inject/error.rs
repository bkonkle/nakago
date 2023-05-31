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

    /// An error indicating that the Provider associated with an Injector wasn't able to downcast
    /// because the types didn't match. This should not be thrown under normal circumstances.
    TypeMismatch(
        /// The Key of the entity type with a Provider that doesn't match
        Key,
    ),

    /// An error thrown from a Provider
    Provider(#[from] anyhow::Error),

    /// The given Key was unable to be removed from the Inject container
    CannotConsume(
        /// The Key of the type that consumption was attempted for
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
            Self::TypeMismatch(key) => write!(f, "the type for {key} does not match"),
            Self::Provider(_) => write!(f, "provider failure"),
            Self::CannotConsume(key) => write!(f, "{key} was not able to be consumed"),
        }
    }
}
