//! # Episodes
#![forbid(unsafe_code)]

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Mutations
pub mod mutations;

/// GraphQL Queries
pub mod queries;

/// GraphQL Resolver
pub mod resolver;

/// GraphQL Schema
pub mod schema;

/// Service
pub mod service;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loaders::{Loader, LOADER};
pub use resolver::{EpisodesMutation as Mutation, EpisodesQuery as Query};
pub use service::{Service, SERVICE};
