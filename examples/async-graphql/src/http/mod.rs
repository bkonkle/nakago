/// Health handlers
pub mod health;

/// GraphQL handlers
pub mod graphql;

/// Events handlers
pub mod events;

/// Init all handlers
pub mod init;

pub use init::{Init, Load};
