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

## Installation

### Cargo

- Install Rust and Cargo by following [this guide](https://www.rust-lang.org/tools/install).
- Run `cargo install nakago`

## Development

To set up a development environment to build this project, you'll need to install some helpful tools.

### Clippy

For helpful linting rools, install [Clippy](https://github.com/rust-lang/rust-clippy)

Run it with `cargo`:

```sh
cargo clippy --fix
```

If you're using VS Code, configure the `rust-analyzer` plugin to use it (in _settings.json_):

```json
{
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

### pre-commit

Install pre-commit to automatically set up Git hook scripts.

In Ubuntu, the package to install is `pre-commit`:

```sh
sudo apt install pre-commit
```

On Mac with Homebrew, the package is also `pre-commit`:

```sh
brew install pre-commit
```

### libclang

The `cargo-spellcheck` utility depends on [`libclang`](https://clang.llvm.org/doxygen/group__CINDEX.html).

In Ubuntu, the package to install is `libclang-dev`:

```sh
sudo apt install libclang-dev
```

### Cargo Make

To use build scripts from the _Makefile.toml_, install Cargo Make:

```sh
cargo install cargo-make
```

Run "setup" to install some tooling dependencies:

```sh
cargo make setup
```

### Running the Local dev server

Use `cargo` to run the dev server locally:

```sh
cargo make dev
```

### Update Dependencies

First, install the `outdated` command for `cargo`:

```sh
cargo install cargo-outdated
```

Then, update and check for any major dependency changes:

```sh
cargo update
cargo outdated
```

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Attribution

Katana image by fordevector at [Vecteezy](https://www.vecteezy.com/free-vector/katana).
