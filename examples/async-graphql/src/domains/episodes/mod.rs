//! # Episodes
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

/// Episode DataLoader
pub mod loader;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loader::{EpisodeLoader, Provider as EpisodeLoaderProvider, EPISODE_LOADER};
pub use service::{
    DefaultService as DefaultEpisodesService, Provider as EpisodesServiceProvider,
    Service as EpisodesService, EPISODES_SERVICE,
};

#[cfg(test)]
pub use service::MockService as MockEpisodesService;
