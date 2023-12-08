use std::{fmt::Debug, sync::Arc};

use backtrace::Backtrace;
use thiserror::Error;

use super::{hooks, provider, Key};

/// A Dependency Injection Result
pub type Result<T> = std::result::Result<T, Error>;

/// Injection Errors
#[derive(Error, Debug, Clone)]
pub enum Error {
    /// Type ID already occupied
    #[error("{0} has already been provided")]
    Occupied(
        /// The Key of the entity type that was already provided
        Key,
    ),

    /// An instance for the given Key was not found
    #[error("{missing} was not found\n\nAvailable:{}\n{}", format_avail_lines(.available), format_backtrace(.backtrace))]
    NotFound {
        /// The Key of the entity that was not found
        missing: Key,

        /// The Keys that are available in the container
        available: Vec<Key>,

        /// A Backtrace of the error
        backtrace: Arc<Backtrace>,
    },

    /// An error thrown from a Provider
    #[error("provider failure")]
    Provider(#[from] Box<provider::Error>),

    /// An error thrown from a Hook
    #[error("hook failure")]
    Hook(#[from] Box<hooks::Error>),

    /// An error thrown when an Any type cannot be downcast to the given concrete type
    #[error("{0} was not able to be downcast to {}", .0.type_name)]
    TypeMismatch(
        /// The Key of the entity that was not found
        Key,
    ),

    /// An error thrown when a Key cannot be consumed and removed from the container, usually
    /// because there are still active refs to the Arc containing the dependency.
    #[error("{key} cannot be consumed, {strong_count} strong pointers remain")]
    CannotConsume {
        /// The Key of the entity that cannot be consumed
        key: Key,

        /// The number of outstanding refs to the dependency
        strong_count: usize,
    },
}

fn format_avail_lines(available: &Vec<Key>) -> String {
    if !available.is_empty() {
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
    }
}

fn format_backtrace(backtrace: &Arc<Backtrace>) -> String {
    match std::env::var("RUST_LIB_BACKTRACE").or_else(|_| std::env::var("RUST_BACKTRACE")) {
        Ok(should_disable) if should_disable != "0" => {
            format!("\nstack backtrace:\n{:?}", backtrace)
        }
        _ => "".to_string(),
    }
}

/// Convert a Provider error into an Inject error
pub fn from_provider_error(e: provider::Error) -> Error {
    Error::Provider(Box::new(e))
}

/// Convert a Hook error into an Inject error
pub fn from_hook_error(e: hooks::Error) -> Error {
    Error::Hook(Box::new(e))
}
