# Development

To set up a development environment to build this project, you'll need to install some helpful tools.

## Clippy

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

## pre-commit

Install pre-commit to automatically set up Git hook scripts.

In Ubuntu, the package to install is `pre-commit`:

```sh
sudo apt install pre-commit
```

On Mac with Homebrew, the package is also `pre-commit`:

```sh
brew install pre-commit
```

## libclang

The `cargo-spellcheck` utility depends on [`libclang`](https://clang.llvm.org/doxygen/group__CINDEX.html).

In Ubuntu, the package to install is `libclang-dev`:

```sh
sudo apt install libclang-dev
```

## Cargo Make

To use build scripts from the _Makefile.toml_, install Cargo Make:

```sh
cargo install cargo-make
```

Run "setup" to install some tooling dependencies:

```sh
cargo make setup
```

## Running the Local dev server

Use `cargo` to run the dev server locally:

```sh
cargo make dev
```

## Update Dependencies

First, install the `outdated` command for `cargo`:

```sh
cargo install cargo-outdated
```

Then, update and check for any major dependency changes:

```sh
cargo update
cargo outdated
```
