/// The dependency injection container
pub mod container;

/// Providers
pub mod provide;

/// Errors
pub mod error;

/// Keys
pub mod key;

/// Tagged dependencies
pub mod tag;

pub use container::Inject;
pub use error::{Error, Result};
pub use key::{Id, Key};
pub use provide::Provider;
