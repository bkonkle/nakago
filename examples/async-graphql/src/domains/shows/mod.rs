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

/// GraphQL Schema
pub mod schema;

/// DataLoaders
pub mod loaders;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

/// Tests
#[cfg(test)]
mod tests;
