//! # Shows

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Mutations
pub mod mutations;

/// GraphQL Resolver
pub mod resolver;

/// GraphQL Schema
pub mod schema;

/// Service
pub mod service;

/// GraphQL Queries
pub mod queries;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loaders::{Loader, LOADER};
pub use resolver::{ShowsMutation as Mutation, ShowsQuery as Query};
pub use service::{Service, SERVICE};
