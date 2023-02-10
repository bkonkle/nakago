<div align="center">

  <img src="https://raw.githubusercontent.com/bkonkle/nakago/main/docs/images/katana.png" width="400" alt="A katana leaning on a stand"/>

  <h1>Nakago (中子)</h1>

  <p>
    <strong>A lightweight Rust framework for elegant services</strong>
  </p>

</div>

[![Crates.io](https://img.shields.io/crates/v/nakago.svg)](https://crates.io/crates/nakago)
[![Docs.rs](https://docs.rs/nakago/badge.svg)](https://docs.rs/nakago)
[![CI](https://github.com/bkonkle/nakago/workflows/CI/badge.svg)](https://github.com/bkonkle/nakago/actions)
[![Coverage Status](https://coveralls.io/repos/github/bkonkle/nakago/badge.svg?branch=main)](https://coveralls.io/github/bkonkle/nakago?branch=main)

## ⚠️ Alpha Disclaimer

NOTE: This library is in early development, and the API may shift rapidly as it evolves. Be advised that this is not yet recommended for Production use.

## Features

- [Dependency Injection](docs/dependency-injection.md)
- HTTP Adapter using [Axum](https://github.com/tokio-rs/axum) (upcoming)
- GraphQL Adapter using [Async-GraphQL](https://github.com/async-graphql/async-graphql) (upcoming)
- CQRS Adapter using [CQRS-ES](https://crates.io/crates/cqrs-es) (upcoming)

As development progresses, major features will be split up into separate crates, so that developers can install only what they need.

## Installation

### Cargo

- Install Rust and Cargo by following [this guide](https://www.rust-lang.org/tools/install).
- Run `cargo install nakago`

## Etymology

A Nakago (中子) is a Japanese word meaning "core", or less commonly the "middle of a nest of boxes". It often refers to the [tang](https://en.wikipedia.org/wiki/Tang_(tools)) of a Japanese Katana - the foundation of the hilt and the mechanism through which a sword is wielded. The nakago must be sound and resilient, allowing the holder to guide the blade with confidence.

## Development

See [docs/development.md](docs/development.md).

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Attribution

Katana image by fordevector at [Vecteezy](https://www.vecteezy.com/free-vector/katana).
