/// The dependency injection container
pub mod container;

/// Errors
pub mod error;

/// Keys
pub mod key;

/// Tagged dependencies
pub mod tag;

/// TypeId Dependencies
pub mod type_id;

/// Hooks
pub mod hooks;

pub use container::{to_provider_error, Inject, Provider};
pub use error::{Error, Result};
pub use hooks::Hook;
pub use key::{Id, Key};
pub use tag::Tag;
