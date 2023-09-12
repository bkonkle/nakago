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
│  │  ├─ handlers.rs
│  │  ├─ mod.rs
│  │  ├─ routes.rs
│  │  └─ state.rs
│  ├─ config.rs -- Your app's custom Config struct
│  ├─ init.rs -- App initialization
│  ├─ lib.rs
│  └─ main.rs -- Main entry point
├─ Cargo.toml
├─ Makefile.toml
├─ README.md
└─ // ...
```

This includes a simple `AppConfig` struct with an embedded `HttpConfig` provided by the `nakago-axum` library. You can add your own configuration fields to this struct and they'll be populated by the [figment](https://docs.rs/figment/latest/figment/) crate.

It includes a barebones `init::app()` function that will load your configuration and initialize your dependencies. You can add your own dependencies to this function and they'll be available when you build your Axum route state.

The `main.rs` uses the [pico-args](https://docs.rs/pico-args/0.5.0/pico_args/) to parse a simple command-line argument to specify an alternate config path, which is useful for many deployment scenarios that dynamically map a config file to a certain mount point within a container filesystem.

In the `http/` folder, you'll find an empty AppState with a dependency injection Provider that you can fill in with your own dependencies. The router maps a simple `GET /health` route to a handler that returns a JSON response with a success message.

You now have a simple foundation to build on. Let's add some more functionality!

## Setup

Follow the Installation instructions in the `README.md` to prepare your new local environment.

## Authentication

One of the first things you'll probably want to add to your application is authentication, which establishes the user's identity. This is separate and distinct from authorization, which determines what the user is allowed to do.

The only currently supported method of authentication is through JWT with JWKS keys. The `nakago-axum` library provides a request extension for for Axum that will use [biscuit](https://docs.rs/biscuit/0.6.0/biscuit/) to decode a JWT from the `Authorization` header, validate it with a JWKS key from the `/.well-known/jwks.json` path on the auth url, and then return the value of the `sub` claim from the payload.

*Configurable claims and other authentication methods will be added in the future.*

### AuthConfig

In your `config.rs` file, add a new property to the `AppConfig` struct:

```rust
use nakago_axum::auth::config::AuthConfig,

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct AppConfig {
    /// HTTP config
    pub http: HttpConfig,

    /// HTTP Auth Config
    pub auth: AuthConfig,
}
```

This `AuthConfig` is automatically loaded as part of the default config loaders in the `nakago-axum` crate, so this line in the `init.rs` ensures that it is populated from environment variables or the currently chosen config file:

```rust
// Config

app.on(
    &EventType::Load,
    AddConfigLoaders::new(default_http_config_loaders()),
);
```

Next, add the following to your `config/default.toml` file:

```toml
[auth]
url = "https://simple-dev.oauth-service.com"
audience = "localhost"

[auth.client]
```

Then, add a hint to your `config/local.toml.example` so that new developers know they need to reach out to you for real values when they create their own `config/local.toml` file:

```toml
[auth.client]
id = "client_id"
secret = "client_secret"
```

Add the real details to your own `config/local.toml` file, which should be excluded from git via the `.gitignore` file.

### Axum State

Before you can use the `Subject` request extension with your Axum routes, you'll need to add the `AuthState` to your `AppState` in your `http/state.rs` file:

```rust
use nakago_axum::auth::authenticate::AuthState;

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct AppState {
    auth: AuthState,
}
```

In your `init.rs` file, you'll want to add the use `ProvideJwks` and `ProvideAuthState` to provide the AuthState that the custom `ProvideAppState` provider unique to your app will use to populate that property.

```rust
// Dependencies

app.provide(&JWKS, ProvideJwks::default().with_config_tag(&CONFIG)).await?;
app.provide(&AUTH_STATE, ProvideAuthState::default()).await?;
```

The `.with_config_tag(&CONFIG)` provides the custom Tag for your `AppConfig`, which will be unique to your app.

Then you can update the `ProvideAppState` provider in `http/state.rs` to retrieve the `AuthState` you provided earlier and use it when creating the `AppState`:

```rust
#[Provider]
#[async_trait]
impl Provider<AppState> for ProvideAppState {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<AppState>> {
        let auth = i.get(&AUTH_STATE).await?;

        Ok(Arc::new(AppState {
            auth: (*auth).clone(),
        }))
    }
}
```

The `(*auth).clone()` is because the `AuthState`` initially comes wrapped in an Arc, and this easily clones it out of the Arc.

You'll probably also want to add a note to the docstring for your `ProvideAppState` provider so that you can see at a glance that this is a dependency your AppState requires.

```rust
/// **Depends on:**
///   - `Tag(AuthState)`
```

### Axum Route

You can now add a quick handler that allows a user to view their own username when logged in.

```rust
// Get Username
// ------------

/// A Username Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameResponse {
    /// The Status code
    code: usize,

    /// The username, or "(anonymous)"
    username: String,
}

/// Handle Get Username requests
pub async fn get_username_handler(sub: Subject) -> Json<UsernameResponse> {
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

The `Subject` extension uses the AuthState to decode the JWT and return the `sub` claim from the payload. If the user is not logged in, the `Subject` will contain a `None`.

Now add a route that uses the handler to `http/routes.rs`:

```rust
/// Initialize the User route
pub fn new_user_route(_: Inject) -> Route<AppState> {
    Route::new("/", Router::new().route("/username", get(get_username_handler)))
}
```

The `Inject` container is there if you need it, but most things will be provided as part of the `AppState` so you can ignore it.

Finally, in your `init.rs` add a new `InitRoute` hook to your app:

```rust
// Routes

app.on(&EventType::Init, InitRoute::new(new_health_route));
app.on(&EventType::Init, InitRoute::new(new_user_route)); // <-- the new route
```

### Running the App

At this point, you can run your app and see the `(anonymous)` response at the `GET /username` endpoint:

```sh
cargo make run
```

You should see output that looks like the following:

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

Now that you have a simple route that requires authentication, you'll want to add some integration tests to ensure that it works as expected. You don't actually need to have an OAuth2 provider running to test this, because the `nakago-axum` library provides a mock `AuthState` that you can use to simulate a logged-in user.
