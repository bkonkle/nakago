//! # Shows

/// Service
pub mod service;

/// Model
pub mod model;

/// GraphQL Queries
pub mod queries;

/// GraphQL Mutations
pub mod mutations;

/// GraphQL Resolver
pub mod resolver;

/// Show DataLoader
pub mod loader;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loader::{Provider as ShowLoaderProvider, ShowLoader, SHOW_LOADER};
pub use service::{
    DefaultService as DefaultShowsService, Provider as ShowsServiceProvider,
    Service as ShowsService, SHOWS_SERVICE,
};

#[cfg(test)]
pub use service::MockService as MockShowsService;
