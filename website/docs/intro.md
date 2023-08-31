---
sidebar_position: 1
---

# Welcome to Nakago

Nakago is a lightweight framework for building Rust applications with a modular structure, taking advantage of dependency injection and lifecycle events to bring organization and testability to Rust projects large and small.

## ⚠️ Alpha Disclaimer

NOTE: This library is in early development, and the API may shift rapidly as it evolves. Be advised that this is not yet recommended for Production use.

## Features

- [Dependency Injection](features/dependency-injection)
- [HTTP Adapter](features/axum-http) using [Axum](https://github.com/tokio-rs/axum)
- [SQL Adapter](features/sea-orm) using [SeaORM](https://github.com/SeaQL/sea-orm)
- [GraphQL Adapter](features/async-graphql) using [Async-GraphQL](https://github.com/async-graphql/async-graphql)
- CQRS Adapter using [CQRS-ES](https://crates.io/crates/cqrs-es) (upcoming)

## Installation

- Install Rust and Cargo by following [this guide](https://www.rust-lang.org/tools/install).
- Run `cargo install nakago`, along with `cargo install nakago-derive`, `cargo install nakago-axum`, etc. for each feature you need.

## Tutorial

A quick 10-minute [tutorial](tutorial) is coming soon...

## Etymology

Nakago (中子) is a Japanese word meaning "core", or less commonly the "middle of a nest of boxes". It often refers to the [tang](<https://en.wikipedia.org/wiki/Tang_(tools)>) of a Japanese katana - the foundation of the hilt and the mechanism through which a sword is wielded. The nakago must be sound and resilient, allowing the holder to guide the blade with confidence.
