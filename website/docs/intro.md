---
sidebar_position: 1
---

# Welcome to Nakago

Nakago is a toolkit for building Rust applications with a modular structure, taking advantage of dependency injection to bring organization and testability to Rust projects large and small.

## Features

- [Dependency Injection](https://nakago.dev/docs/features/dependency-injection)
- [HTTP Adapter](https://nakago.dev/docs/features/axum-http) for [Axum](https://github.com/tokio-rs/axum) and [Warp](https://github.com/seanmonstar/warp)
- [SQL Adapter](https://nakago.dev/docs/features/sea-orm) for [SeaORM](https://github.com/SeaQL/sea-orm)
- [GraphQL Adapter](https://nakago.dev/docs/features/async-graphql) for [Async-GraphQL](https://github.com/async-graphql/async-graphql)

## Installation

### Cargo

- Install Rust and Cargo by following [this guide](https://www.rust-lang.org/tools/install).
- Run `cargo install nakago`, along with `cargo install nakago-derive`, `cargo install nakago-axum`, etc. for each feature you need.

## Etymology

Nakago (中子) is a Japanese word meaning "core", or less commonly the "middle of a nest of boxes". It often refers to the [tang](<https://en.wikipedia.org/wiki/Tang_(tools)>) of a Japanese katana - the foundation of the hilt and the mechanism through which a sword is wielded. The nakago must be sound and resilient, allowing the holder to guide the blade with confidence.

## Development

See [docs/development.md](https://nakago.dev/docs/development).

## License

Licensed under the MIT license ([LICENSE](https://github.com/bkonkle/nakago/blob/main/LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)).

## Contribution

See [CONTRIBUTING.md](https://github.com/bkonkle/nakago/blob/main/CONTRIBUTING.md).
