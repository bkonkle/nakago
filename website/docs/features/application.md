---
sidebar_position: 2
---

# Application Lifecycle

To manage the lifecycle of an application, the top-level `nakago::Application` struct provides a set of lifecycle hooks and an injection container that can be used to initialize and start the application.

Applications carry a reference to the custom `Config` type that your project uses (and an optional Tag to refer to it if you need multiple Config instances). This Config borrows [Axum](https://github.com/tokio-rs/axum)'s [FromRef](https://docs.rs/axum/latest/axum/extract/trait.FromRef.html) strategy to allow the framework to find pieces of the config it needs embedded in the custom structure that works best for your program.

```rust
/// Server Config
#[derive(Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,

    /// HTTP Auth Config
    pub auth: auth::Config,

    /// Database config
    pub database: nakago_sea_orm::Config,
}
```

## Lifecycle Hooks

Hooks are invoked when a [lifecycle event](https://github.com/bkonkle/nakago/blob/main/nakago/src/lifecycle.rs) is triggered.

### Load

The `Load` event is triggered before the Application loads dependencies and configuration. During this phase, Hooks should provide any dependencies or config loaders that are necessary to initialize and start the App.

### Init

The `Init` event is triggered before the dependencies and configuration are initialized. During this phase, Hooks should perform any initialization steps and construct anything necessary to start the App.

### Startup

The `Startup` event is triggered after the Config is loaded, but before the Application is run. During this phase, the Application should start any background tasks or other long-running processes necessary to keep the App running.

### Shutdown

The `Shutdown` event is triggered before the Application shuts down. During this phase, Hooks should perform any cleanup necessary to cleanly stop the App.

## Starting the Application

To start your application, pass in your top-level Config type and create an instance. Attach Hooks in the order that they should be executed:

```rust
let mut app = AxumApplication::<Config>::default();
app.on(&EventType::Load, users::schema::Load::default());
app.on(&EventType::Load, authz::Load::default());
app.on(&EventType::Init, routes::Init::new(new_health_route));
```

Then, use the underlying server library - Axum in this example - to start listening:

```rust
let (server, addr) = app.run(args.config_path).await?;

info!("Started on port: {port}", port = addr.port());

server.await?;
```
