/// The dependency injection container
pub mod container;

/// Errors
pub mod error;

/// Hooks
pub mod hooks;

/// Keys
pub mod key;

// Injection Providers
pub mod providers;

/// Tagged dependencies
pub mod tag;

/// TypeId Dependencies
pub mod type_id;

pub use container::Inject;
pub use error::{Error, Result};
pub use hooks::{Hook, Hooks};
pub use key::{Id, Key};
pub use providers::{to_provider_error, Provider};
pub use tag::Tag;
