//! # nakago-async-graphql: An Async-GraphQL integration for Nakago
#![forbid(unsafe_code)]

/// The GraphQL Schema
pub mod schema;

/// Testing utils
pub mod test;

/// Errors
pub mod errors;

/// Utils
pub mod utils;

/// Error macros
#[macro_use]
extern crate anyhow;
