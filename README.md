<div align="center">

  <img src="https://raw.githubusercontent.com/bkonkle/nakago/main/website/static/img/katana.png" width="400" alt="A katana leaning on a stand"/>

  <h1>Nakago (中子)</h1>

  <p>
    <strong>A lightweight async dependency injection toolkit for Rust</strong>
  </p>

[![Crates.io](https://img.shields.io/crates/v/nakago.svg)](https://crates.io/crates/nakago)
[![Docs.rs](https://docs.rs/nakago/badge.svg)](https://docs.rs/nakago)
[![CI](https://github.com/bkonkle/nakago/workflows/CI/badge.svg)](https://github.com/bkonkle/nakago/actions)
[![Coverage Status](https://codecov.io/gh/bkonkle/nakago/branch/main/graph/badge.svg?token=BXEZAMHVLP)](https://codecov.io/gh/bkonkle/nakago)

[![Rust](https://img.shields.io/badge/rust-2021-a72145?logo=rust&style=flat)](https://www.rust-lang.org)
[![Tokio](https://img.shields.io/badge/tokio-463103?logo=rust&style=flat)](https://tokio.rs)
[![Axum](https://img.shields.io/badge/axum-7b5312?logo=rust&style=flat)](https://crates.io/crates/axum)

</div>

Nakago is a toolkit for building Rust applications with a modular structure, taking advantage of dependency injection to bring organization and testability to Rust projects large and small.

## Archived

I'm not working on Nakago anymore, so I've archived the repo.

I built Nakago to bring dependency injection to Rust apps — a typed container you register providers in, so tests and different environments can swap pieces out without a lot of rewiring. When I picked it back up recently for a new service, I tried the same setup both ways: once with Nakago, and once in plain Rust with a small composition trait per backend, an enum that picks one at startup, and a struct of trait objects for the services. I think the plain Rust version won pretty clearly. It came out a little shorter, a missing dependency became a compile error instead of a runtime one, and it didn't pull in twenty-odd extra crates. The pieces I most wanted to swap were generic over traits that can't be boxed anyway, so a container couldn't hide them.

So for the apps I'm building now, plain Rust composition does the job, and I'd rather not keep a framework around that I'm not using. The code and the crates on crates.io stay up for reference, but they won't get updates, and `main` no longer builds — one of its git dependencies is gone. If you're using Nakago, I'd move the wiring into plain structs and traits. It's less code than it sounds like. Thanks to everyone who tried it out.

## Features

- [Dependency Injection](https://nakago.dev/docs/features/dependency-injection)
- [HTTP Adapter](https://nakago.dev/docs/features/axum-http) for [Axum](https://github.com/tokio-rs/axum) and [Warp](https://github.com/seanmonstar/warp)
- [SQL Adapter](https://nakago.dev/docs/features/sea-orm) for [SeaORM](https://github.com/SeaQL/sea-orm)
- [GraphQL Adapter](https://nakago.dev/docs/features/async-graphql) for [Async-GraphQL](https://github.com/async-graphql/async-graphql)
- CQRS Adapter using [CQRS-ES](https://crates.io/crates/cqrs-es) (upcoming)

As development progresses, components will be split up into separate crates, so that developers can install only what they need.

## Installation

### Cargo

- Install Rust and Cargo by following [this guide](https://www.rust-lang.org/tools/install).
- Run `cargo install nakago`, along with `cargo install nakago-derive`, `cargo install nakago-axum`, etc. for each feature you need.

## Etymology

Nakago (中子) is a Japanese word meaning "core", or less commonly the "middle of a nest of boxes". It often refers to the [tang](<https://en.wikipedia.org/wiki/Tang_(tools)>) of a Japanese katana - the foundation of the hilt and the mechanism through which a sword is wielded. The nakago must be sound and resilient, allowing the holder to guide the blade with confidence.

## Development

See [docs/development.md](https://nakago.dev/docs/development).

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>).

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Attribution

Katana image by fordevector at [Vecteezy](https://www.vecteezy.com/free-vector/katana).
