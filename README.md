<div align="center">

  <img src="https://raw.githubusercontent.com/bkonkle/nakago/main/website/static/img/katana.png" width="400" alt="A katana leaning on a stand"/>

  <h1>Nakago (中子)</h1>

  <p>
    <strong>A lightweight Rust framework for sharp services</strong>
  </p>

[![Crates.io](https://img.shields.io/crates/v/nakago.svg)](https://crates.io/crates/nakago)
[![Docs.rs](https://docs.rs/nakago/badge.svg)](https://docs.rs/nakago)
[![CI](https://github.com/bkonkle/nakago/workflows/CI/badge.svg)](https://github.com/bkonkle/nakago/actions)
[![Coverage Status](https://codecov.io/gh/bkonkle/nakago/branch/main/graph/badge.svg?token=BXEZAMHVLP)](https://codecov.io/gh/bkonkle/nakago)

[![Rust](https://img.shields.io/badge/rust-2021-a72145?logo=rust&style=flat)](https://www.rust-lang.org)
[![Tokio](https://img.shields.io/badge/tokio-463103?logo=rust&style=flat)](https://tokio.rs)
[![Axum](https://img.shields.io/badge/axum-7b5312?logo=rust&style=flat)](https://crates.io/crates/axum)

</div>

Nakago is a lightweight framework for building Rust applications with a modular structure, taking advantage of dependency injection and lifecycle events to bring organization and testability to Rust projects large and small.

## ⚠️ Alpha Disclaimer

NOTE: This library is in early development, and the API may shift rapidly as it evolves. Be advised that this is not yet recommended for Production use.

## Features

- [Dependency Injection](https://bkonkle.github.io/nakago/docs/features/dependency-injection)
- [HTTP Adapter](https://bkonkle.github.io/nakago/docs/features/axum-http) using [Axum](https://github.com/tokio-rs/axum)
- [SQL Adapter](https://bkonkle.github.io/nakago/docs/features/sea-orm) using [SeaORM](https://github.com/SeaQL/sea-orm)
- [GraphQL Adapter](https://bkonkle.github.io/nakago/docs/features/async-graphql) using [Async-GraphQL](https://github.com/async-graphql/async-graphql)
- CQRS Adapter using [CQRS-ES](https://crates.io/crates/cqrs-es) (upcoming)

As development progresses, major features will be split up into separate crates, so that developers can install only what they need.

## Installation

### Cargo

- Install Rust and Cargo by following [this guide](https://www.rust-lang.org/tools/install).
- Run `cargo install nakago`, along with `cargo install nakago-derive`, `cargo install nakago-axum`, etc. for each feature you need.

## Etymology

Nakago (中子) is a Japanese word meaning "core", or less commonly the "middle of a nest of boxes". It often refers to the [tang](<https://en.wikipedia.org/wiki/Tang_(tools)>) of a Japanese katana - the foundation of the hilt and the mechanism through which a sword is wielded. The nakago must be sound and resilient, allowing the holder to guide the blade with confidence.

## Development

See [docs/development.md](https://bkonkle.github.io/nakago/docs/development).

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>).

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Attribution

Katana image by fordevector at [Vecteezy](https://www.vecteezy.com/free-vector/katana).
