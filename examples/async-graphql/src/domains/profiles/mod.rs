//! # Profiles
#![forbid(unsafe_code)]

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

/// Profile DataLoader
pub mod loader;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loader::{ProfileLoader, Provider as ProfileLoaderProvider, PROFILE_LOADER};
pub use service::{
    DefaultService as DefaultProfilesService, Provider as ProfilesServiceProvider,
    Service as ProfilesService, PROFILES_SERVICE,
};

#[cfg(test)]
pub use service::MockService as MockProfilesService;
