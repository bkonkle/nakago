//! # nakago-ws: A Warp HTTP routes integration for Nakago
#![forbid(unsafe_code)]

/// WebSocket Connections
pub mod connections;

/// The WebSocket Controller
pub mod controller;

/// Authentication helpers
pub mod auth;

#[macro_use]
extern crate log;

pub use connections::Connections;
pub use controller::{Controller, Handler};
