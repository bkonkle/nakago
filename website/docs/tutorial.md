---
sidebar_position: 2
---

# Tutorial

This tutorial will walk you through the basics of using Nakago to build a simple HTTP service. It will use Axum to provide HTTP routes and will decode the user's JWT token and verify their identity via a separate OAuth2 provider, such as Auth0 or Okta or your own self-hosted service.

## Cargo-Generate Template

First install `cargo-generate`:

```sh
cargo install cargo-generate
```

Then generate a new project with this template:

```sh
cargo generate bkonkle/nakago-simple-template
```

You'll see a folder structure like this:

```text
simple/
├─ .cargo/ -- Clippy config
├─ .github/ -- Github Actions
├─ config/ -- Config files for different environments
├─ src/
│  ├─ http/ -- Axum HTTP routes
│  │  ├─ mod.rs
│  │  ├─ health.rs -- The HTTP health check handler
│  │  └─ router.rs -- Axum router initialization
│  ├─ lib.rs
│  ├─ config.rs -- Your app's custom Config struct
│  ├─ init.rs -- App initialization
│  └─ main.rs -- Main entry point
├─ Cargo.toml
├─ Makefile.toml
├─ README.md
└─ // ...
```

This includes a simple app-specific `Config` struct with an embedded `http` config provided by the `nakago-axum` library. You can add your own configuration fields to this struct and they'll be populated by the [figment](https://docs.rs/figment/latest/figment/) crate using the [nakago-figment](https://github.com/bkonkle/nakago/tree/main/nakago_figment) library.

It includes a barebones `init::app()` function that will load your configuration and initialize your dependencies. You can add your own dependencies to this function and they'll be available when you build your Axum routes through a convenient `State` struct that contains the injection container.

The `main.rs` file uses [pico-args](https://docs.rs/pico-args/0.5.0/pico_args/) to parse a simple command-line argument to specify an alternate config path, which is useful for many deployment scenarios that dynamically map a config file to a certain mount point within a container filesystem.

In the `http/` folder, you'll find an Axum handler and a router initialization function. The router maps a simple `GET /health` route to a handler that returns a JSON response with a success message.

You now have a simple foundation to build on. Let's add some more functionality!

## Setup

Follow the Installation instructions in the `README.md` to prepare your new local environment.

## Authentication

One of the first things you'll probably want to add to your application is authentication, which establishes the user's identity. This is separate and distinct from authorization, which determines what the user is allowed to do.

The only currently supported method of authentication is through JWT with JWKS keys, though other methods will be added in the future. The `nakago-axum` library provides a request extractor for for Axum that uses [biscuit](https://docs.rs/biscuit/0.6.0/biscuit/) with your Figment Config to decode a JWT from the `Authorization` header, validate it with a JWKS key from the `/.well-known/jwks.json` path on the auth url, and then return the value of the `sub` claim from the payload.

*Configurable claims and other authentication methods will be added in the future.*

### Auth Config

In your `config.rs` file, add a new property to the app's `Config` struct:

```rust
use nakago_axum::auth;
// ...

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// HTTP Auth Config
    pub auth: auth::Config,
}
```

This auth `Config` is automatically loaded as part of the default config loaders in the `nakago-axum` crate, which you'll see below.

Next, add the following values to your `config.local.toml.example` file as a hint, so that new developers know they need to reach out to you for real values when they create their own `config.local.toml` file:

```toml
[auth]
url = "https://simple-dev.oauth-service.com"
audience = "localhost"

[auth.client]
id = "client_id"
secret = "client_secret"
```

Add the real details to your own `config.toml` file, which should be excluded from git via the `.gitignore` file. If you don't have real values yet, leave them as the dummy values above. You can still run integration tests without having a real OAuth2 provider running, if you want.

### Initialization

You're now ready to head over to your initialization routine. This is where you will provide all of the dependencies and setup your app needs in order to run.

This block that is already in the top-level `init.rs` ensures your config is populated from environment variables or the currently chosen config file, along with the auth property you added above:

```rust
// These lines should already be in your `init.rs` file - no change needed
nakago_figment::Init::<Config>::default()
    .maybe_with_path(config_path)
    .init(&i)
    .await?;
```

First, add the default JWKS Validator from `nakago_axum`'s `auth` module using the `provide` method, which uses the type as the key for the Inject container. Add this to your `init.rs` file, within the `app()` function:

```rust
use nakago_axum::auth::{validator, Validator};

// ...

i.provide::<Box<dyn Validator>>(validator::Provide::default())
        .await?;
```

This will be overridden in your tests to use the unverified variant, but we'll get to that later. Next you should use `jwks::Provide` to inject the JWKS config. Add thios to your `init.rs` file as well:

```rust
use nakago_axum::auth::{jwks, JWKSet, Empty};

// ...

i.provide::<JWKSet<Empty>>(jwks::Provide::<Config>::default())
        .await?;
```

Your `init.rs` should now look like this:

```rust
use std::path::PathBuf;

use nakago::{Inject, Result};
use nakago_axum::{
    auth::{jwks, validator, Empty, JWKSet, Validator},
    config,
};

use crate::config::Config;

/// Create a dependency injection container for the top-level application
pub async fn app(config_path: Option<PathBuf>) -> Result<Inject> {
    let i = Inject::default();

    i.provide::<Box<dyn Validator>>(validator::Provide::default())
        .await?;

    i.provide::<JWKSet<Empty>>(jwks::Provide::<Config>::default())
        .await?;

    // Add config loaders before the Config is initialized
    config::add_default_loaders(&i).await?;

    // Initialize the Config
    nakago_figment::Init::<Config>::default()
        .maybe_with_path(config_path)
        .init(&i)
        .await?;

    Ok(i)
}
```

### Axum Route

You can now add a quick handler to `http/` that allows a user to view their own username when logged in. Create a new file called `http/user.rs`:

```rust
use axum::Json;
use nakago_axum::auth::Subject;
use serde_derive::{Deserialize, Serialize};

/// A Username Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameResponse {
    /// The Status code
    code: usize,

    /// The username, or "(anonymous)"
    username: String,
}

/// Handle Get Username requests
pub async fn get_username(sub: Subject) -> Json<UsernameResponse> {
    let username = if let Subject(Some(username)) = sub {
        username.clone()
    } else {
        "(anonymous)".to_string()
    };

    Json(UsernameResponse {
        code: 200,
        username,
    })
}
```

Make sure to add the `user.rs` file to your `http/mod.rs` file:

```rust
/// User handlers
pub mod user;
```

The `Subject` extension uses Nakago Axum's State proivider to find the Inject container, which it then uses to grab the JWT config and the Validator instance. It decodes the JWT and returns the `sub` claim from the payload. If the user is not logged in, the `Subject` will contain a `None`.

Now add a route that uses the handler to the Init hook at `http/router.rs`:

```rust
use super::{health, user};

// ...

Router::new()
    // ...
    .route("/username", get(user::get_username))
```

Your `http/router.rs` file should now look like this:

```rust
/// This method should already exist in your `http/router.rs` file
pub fn init(i: &Inject) -> Router {
    Router::new()
        .layer(trace_layer())
        .route("/health", get(health::health_check))
        .route("/username", get(user::get_username)) // <-- add this line
        .with_state(State::new(i.clone()))
}
```

### Running the App

At this point, you can run your app and see the `(anonymous)` response at the `GET /username` endpoint:

```sh
cargo make run
```

The uses cargo-make, a tool to provide enhanced Makefile-like functionality for Rust projects. You can see the configuration in the `Makefile.toml` file.

At first, you'll see a big ugly traceback with the following error message at the top because you don't have a valid autd provider configured:

```sh
thread '<unnamed>' panicked at 'Unable to retrieve JWKS: invalid format'
```

This is okay - you don't have to have a properly configured auth provider to run the integration tests for your app. You can use the "unverified" `AuthState` variant during integration testing, and skip the rest of this section.

If you *do* have a valid OAuth2 provider, then you'll want to create a `config.local.toml` file and set the following property in it:

```toml
[auth]
url = "https://simple-dev.oauth-service.com"
```

You can also use the `AUTH_URL` environment variable to set this value. Consider using a tool like [direnv](https://direnv.net/) to manage variables like this in your local development environment with `.envrc` files.

Your provider should have a `/.well-known/jwks.json` file available at the given auth url, which will avoid the error message above. You should now see output that looks like the following:

```sh
2023-09-08T02:14:03.388670Z  INFO simple: Started on port: 8000
```

When you call `http://localhost:8000/username` in your browser, you should see the following response:

```json
{
  "code": 200,
  "username": "(anonymous)"
}
```

## Integration Testing

Now that you have a simple route that requires authentication, you'll want to add some integration tests to ensure that it works as expected. You don't actually need to have an OAuth2 provider running to test this, because the `nakago-axum` library provides a mock unverified `AuthState` that you can use to simulate a logged-in user.

### Test Utils

Nakago Axum's HTTP `Utils` class is based on the idea of extending the base test `Utils` class you'll find in `nakago_axum::test::Utils` with additional functionality, like adding a `graphql` property if you're using `nakago-async-graphql` or adding convenience methods around your app-specific data.

To start out with, create a `tests` folder alongside your `src`. This will be used by Cargo as an ["integration test"](<https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests>) module, and will be excluded from your final binary. It allows you to import the module in your `src` as if it were an external package, with access only to the public exports. You don't need to add a `lib.rs`, `mod.rs`, or `main.rs` - each file in the `tests` folder will be auto-discovered and treated as a separate entry point with its own module.

For the purposes of your own application, you'll want to create a `tests/utils.rs` file that wraps the `nakago_axum::test::Utils` so that you can override any dependencies that you need or add convenience methods to build test data easily for your tests. Start out with a newtype like this:

```rust
use simple::Config;

pub struct TestUtils(nakago_axum::test::Utils<Config>);
```

Replace `simple` with your actual project name.

To make it easy to access the fields on the inner `TestUtils`, you can implement the `Deref` trait for your newtype. This isn't generally a good practice for newtypes in Production because it can result in some easy-to-miss implicit conversion behind the scenes, but in testing it's a nice convenience:

```rust
use std::ops::Deref;

// ...

impl Deref for TestUtils {
    type Target = nakago_axum::test::Utils<Config>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

Now, you can implement an `init()` method for your app-specific `Utils` wrapper:

```rust
use anyhow::Result;
use nakago_axum::auth::{validator, Validator};
use simple::{http::router, init, Config};

// ...

impl TestUtils {
    pub async fn init() -> Result<Self> {
        let config_path = std::env::var("CONFIG_PATH_SIMPLE")
            .unwrap_or_else(|_| "examples/simple/config.test.toml".to_string());

        let i = init::app(Some(config_path.clone().into())).await?;

        i.replace_with::<Validator>(validator::ProvideUnverified::default())
            .await?;

        let router = router::init(&i);

        let utils = nakago_axum::test::Utils::init(i, "/", router).await?;

        Ok(Self(utils))
    }
}
```

Again, replace `simple` with your actual project name. The `CONFIG_PATH` variable is used so that you can replace that with `config.ci.toml` or whatever you need for testing in different environments.

Now, create a `test_users_int.rs` to represent your User integration tests, which will currently just test the `/username` endpoint.

```rust
#![cfg(feature = "integration")]

use utils::TestUtils;

#[tokio::test]
async fn test_get_username_success() -> Result<()> {
    let utils = TestUtils::init().await?;

    todo!("unimplemented")
}
```

The `#![cfg(feature = "integration")]` at the top of this file means that it will only be included in the build if the `integration` feature flag is enabled. This is a good practice to follow for all your integration tests, because it allows you to run your unit tests while skipping integration tests so that you don't need supporting services in a local Docker Compose formation or other external dependencies.

The `todo!()` macro allows you to leave this test unfinished for now, but it will throw an error if you try to execute the tests.

### HTTP Calls

Next, we can add an HTTP call with a JWT token. First, create the dummy token, which will only work with the `auth::subject::ProvideUnverified` Validator provider above for use in testing.

```rust
use ulid::Ulid;

#[tokio::test]
async fn test_get_username_success() -> Result<()> {
    let utils = TestUtils::init().await?; // <-- this line should already be there

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    todo!("unimplemented")
}
```

Now we can make the HTTP call:

```rust
let resp = utils
    .http
    .get_json("/username", Some(&token))
    .send()
    .await?;
```

Pull the response apart into a status and a JSON body:

```rust
let status = resp.status();
let json = resp.json::<Value>().await?;
```

Now you can make assertions based on the response:

```rust
assert_eq!(status, 200);
assert_eq!(json["username"], username);
```

Add an `Ok(())` at the end to signal a successful test run, and your final test should look like this:

```rust
#![cfg(feature = "integration")]

use anyhow::Result;

#[cfg(test)]
mod utils;

use serde_json::Value;
use ulid::Ulid;
use utils::TestUtils;

#[tokio::test]
async fn test_get_username_success() -> Result<()> {
    let utils = TestUtils::init().await?;

    let username = Ulid::new().to_string();
    let token = utils.create_jwt(&username).await?;

    let resp = utils
        .http
        .get_json("/username", Some(&token))
        .send()
        .await?;

    let status = resp.status();

    let json = resp.json::<Value>().await?;

    assert_eq!(status, 200);
    assert_eq!(json["username"], username);

    Ok(())
}
```

### Running the Tests

To run integration tests locally, add the following command to your `Makefile.toml`:

```toml
[tasks.integration]
env = { "RUN_MODE" = "test", "RUST_LOG" = "info", "RUST_BACKTRACE" = 1 }
command = "cargo"
args = ["nextest", "run", "--features=integration", "--workspace", "${@}"]
```

This won't work until you add the `integration` feature to your `Cargo.toml`, however:

```toml
[features]
integration = []
```

Now you can run `cargo make integration`, and it will use [nextest](https://github.com/nextest-rs/nextest) to run all available integration tests. It also allows you to pass options to `nextest`, including filtering down to a specific test or group of tests.

```sh
cargo make integration
```

You should see a message that looks like the following:

```sh
    Starting 1 test across 4 binaries
        PASS [   0.230s] simple::test_users_int test_get_username_success
------------
     Summary [   0.230s] 1 test run: 1 passed, 0 skipped
```

If you want to see it fail, you can adjust the expectations at the end of the test in `test_users_int.rs`:

```rust
assert_eq!(json["username"], "bob");
```

Instead of the output above, you'll see a gnarly stacktrace with the following at the top:

```sh
        FAIL [   0.378s] simple::test_users_int test_get_username_success

--- STDOUT:              simple::test_users_int test_get_username_success ---

running 1 test
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `String("01HA5SF2AB3FV269P5ZEZ46033")`,
 right: `"bob"`', tests/test_users_int.rs:32:5
```

## Finished Result

Congratulations! You now have a simple API server with JWT+JWKS authentication in Rust, and you've added integration tests to ensure that it works as expected!

You can see everything together in the [examples/simple](https://github.com/bkonkle/nakago/tree/main/examples/simple) folder of the `nakago` repository.
