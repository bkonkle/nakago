//! # Episodes
#![forbid(unsafe_code)]

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Mutations
pub mod mutation;

/// GraphQL Queries
pub mod query;

/// GraphQL Schema
pub mod schema;

/// Service
pub mod service;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loaders::Loader;
pub use mutation::EpisodesMutation as Mutation;
pub use query::EpisodesQuery as Query;
pub use service::Service;
