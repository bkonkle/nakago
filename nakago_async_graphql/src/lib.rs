//! # nakago-async-graphql: An Async-GraphQL integration for Nakago
#![forbid(unsafe_code)]

/// GraphQL data for the resolvers
pub mod data;

/// GraphQL DataLoaders
pub mod loaders;

/// The GraphQL Schema
pub mod schema;

/// Services that support the resolvers
pub mod services;
