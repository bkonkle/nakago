//! # Profiles
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

pub use loaders::LOADER;
pub use mutation::{ProfilesMutation as Mutation, MUTATION};
pub use query::{ProfilesQuery as Query, QUERY};
pub use service::{Service, SERVICE};
