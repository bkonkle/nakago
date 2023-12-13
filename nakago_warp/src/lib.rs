//! # nakago-wary: A Warp HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// The top-level Applicaiton
pub mod app;

/// HTTP config
pub mod config;

/// Errors
pub mod errors;

#[macro_use]
extern crate log;
