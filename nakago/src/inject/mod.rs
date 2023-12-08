/// The dependency injection container
pub mod container;

/// Errors
pub mod errors;

/// Keys
pub mod key;

/// Tagged dependencies
pub mod tag;

/// TypeId Dependencies
pub mod type_id;

/// Hooks
pub mod hooks;

/// Provider
pub mod provider;

/// Injector
pub mod injector;

pub use container::Inject;
pub use errors::{from_hook_error, from_provider_error, Error, Result};
pub use hooks::{to_hook_error, Hook};
pub use injector::{Dependency, Pending};
pub use key::{Id, Key};
pub use provider::{to_provider_error, Provider};
pub use tag::Tag;

pub(crate) use injector::Injector;
