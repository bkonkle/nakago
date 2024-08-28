---
sidebar_position: 5
---

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

In Arch Linux, sync `pre-commit`:

```sh
pacman -S pre-commit
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

In Arch Linux, sync `clang`:

```sh
pacman -S clang
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

## Running Integration Tests

First, create a `.envrc` file by copying `.envrc.example`, and run `direnv allow` to load the environment variables.

Then spin up the supporting Docker Compose processes:

```sh
cargo make docker up -d
```

Now you can reset the test DB:

```sh
cargo make db-reset
```

And then run the integration tests:

```sh
cargo make integration
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

### SQLx CLI

The cargo-make setup task installs the SQLx CLI for running database migrations for the example projects.

Create a database based on the `DATABASE_URL` in the `.envrc`, if you haven't already:

```sh
cargo make db-create
```

Run migrations:

```sh
cargo make db-migrate
```

If you want to wipe your database and start over:

```sh
cargo make db-reset
```

## Examples: Docker Build

To build locally, use Buildkit:

```sh
DOCKER_BUILDKIT=1 docker build -t async-graphql -f examples/async-graphql/Dockerfile .
```

To clear the build cache:

```sh
docker builder prune --filter type=exec.cachemount
```

To inspect the local filesystem:

```sh
docker run --rm -it --entrypoint=/bin/bash async-graphql
```

To inspect the full build context:

```sh
docker image build --no-cache -t build-context -f - . <<EOF
FROM busybox
WORKDIR /build-context
COPY . .
CMD find .
EOF

docker container run --rm build-context
```

And to clean up the build context test image:

```sh
docker image rm build-context
```
