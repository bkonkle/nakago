//! # Shows

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Mutations
pub mod mutation;

/// GraphQL Schema
pub mod schema;

/// Service
pub mod service;

/// GraphQL Queries
pub mod query;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;

pub use loaders::{Loader, LOADER};
pub use mutation::{ShowsMutation as Mutation, MUTATION};
pub use query::{ShowsQuery as Query, QUERY};
pub use service::{Service, SERVICE};
