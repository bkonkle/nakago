/// The dependency injection container
pub mod container;

/// Providers
pub mod provide;

/// Initializers
pub mod initialize;

/// Errors
pub mod error;

/// Keys
pub mod key;

/// Tagged dependencies
pub mod tag;

pub use container::Inject;
pub use error::{Error, Result};
pub use initialize::Initializer;
pub use key::{Id, Key};
pub use provide::Provider;
pub use tag::Tag;
