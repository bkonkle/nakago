//! # Users

/// DataLoaders
pub mod loaders;

/// Model
pub mod model;

/// GraphQL Mutation
pub mod mutation;

/// GraphQL Query
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

pub use loaders::{Loader, LOADER};
pub use mutation::{UsersMutation as Mutation, MUTATION};
pub use query::{UsersQuery as Query, QUERY};
pub use service::{Service, SERVICE};
