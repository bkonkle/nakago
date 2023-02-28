/// The dependency injection container
pub mod container;

/// Providers
pub mod provide;

/// Hooks
pub mod hooks;

/// Errors
pub mod error;

/// Keys
pub mod key;

/// Tagged dependencies
pub mod tag;

/// TypeId Dependencies
pub mod type_id;

pub use container::Inject;
pub use error::{Error, Result};
pub use hooks::{Hook, NoOpHook};
pub use key::{Id, Key};
pub use provide::{to_provider_error, Provider};
pub use tag::Tag;
