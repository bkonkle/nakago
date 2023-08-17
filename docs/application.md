# Applications

To manage the lifecycle of an application, the top-level `nakago::Application` struct provides Init and Startup hooks and a system to trigger them. More hooks - like a Shutdown hook - are coming soon.

Applications are currently very simple:

```rust
pub struct Application<C: Config> {
    events: Events,
    i: inject::Inject,
    _phantom: PhantomData<C>,
}
```

First, they carry a PhantomData reference to the custom `Config` type that your project uses. This Config borrows [Axum](https://github.com/tokio-rs/axum)'s [FromRef](https://docs.rs/axum/latest/axum/extract/trait.FromRef.html) strategy to allow the framework to find pieces of the config it needs embedded in the custom structure that works best for your program.

```rust
/// Server Config
#[derive(Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct AppConfig {
    /// HTTP config
    pub http: HttpConfig,

    /// HTTP Auth Config
    pub auth: AuthConfig,

    /// Database config
    pub database: DatabaseConfig,
}
```

## Lifecycle Hooks

Hooks are invoked when a [lifecycle event](https://github.com/bkonkle/nakago/blob/main/nakago/src/lifecycle.rs) is triggered.

### Init

The `Init` Hook is invoked before the Config Loaders are requested from the container and the loaders are used to initialize the Config. Anything that is needed to initialize your application's Config should go here. Each Hook is triggered in order when a lifecycle event is triggered.

### Startup

The `Startup` Hook is invoked after the Config is loaded, but before the Application is run.

## Starting the Application

To start your application, pass in your top-level Config type and create an instance. Attach Hooks in the order that they should be executed:

```rust
let mut app = AxumApplication::<AppConfig>::default();
app.on(&EventType::Init, InitDomains::default());
app.on(&EventType::Init, InitApp::default());
app.on(&EventType::Startup, InitAuthz::default());
```

Then, use the underlying server library - Axum in this example - to start listening:

```rust
let server = app.run::<AppState>(args.config_path).await?;
let addr = server.local_addr();

info!("Started on port: {port}", port = addr.port());

server.await?;
```
