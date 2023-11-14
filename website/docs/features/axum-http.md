---
sidebar_position: 3
---

# Axum HTTP Applications

The `nakago-axum` crate defines `AxumApplication`, which wraps `Application` and provides a way to Run an HTTP service and use the `Inject` container via Axum's `State` mechanism.

## Dependencies vs. Context

Axum provides the State extractor that allows you to inject dependencies that stay the same across many requests. For Nakago applications, however, your dependencies are provided through the Inject container. Nakago Axum apps use State to automatically carry the Inject container, but in a way that you don't have to think about while building typicaly applications.

## Application Lifecycle

### Init

The `Init` Hook for an Axum Application automatically adds the `http::Config` and `auth::Config` Loaders before the user-provided hook is invoked and the Config is generated.

### Startup

The `Startup` Hook for an Axum Application uses the State with the provided Router, allowing the flow of the application to proceed through the Axum request handlers.

## Routes

Routes are initialized on Init. There are multiple options for how to implement handlers and access their Dependencies. The approach with the smoothest Nakago integration also has the benefit of allowing you to define Controller structs with methods that can be used as handlers, allowing you to share common dependencies between related Axum request handlers.

### Controllers

Start with a hypothetical Controller for a WebSocket connection, that will handle requests to upgrade an HTTP request to a WebSocket connection:

```rust
pub const CONTROLLER: Tag<Controller> = Tag::new("events::Controller");

#[derive(Clone)]
pub struct Controller {
    users: Arc<Box<dyn users::Service>>,
    handler: Arc<socket::Handler>,
}
```

It has a dependency on the Users Service and the Socket Handler, which are both used within the "upgrade()" method:

```rust
impl Controller {
    /// Create a new Events handler
    pub async fn upgrade(
        self: Arc<Self>,
        sub: Subject,
        ws: WebSocketUpgrade,
    ) -> axum::response::Result<impl IntoResponse> {
        // Retrieve the request User, if username is present
        let user = if let Subject(Some(ref username)) = sub {
            self.users
                .get_by_username(username, &true)
                .await
                .unwrap_or(None)
        } else {
            None
        };

        Ok(ws.on_upgrade(|socket| async move { self.handler.handle(socket, user).await }))
    }
}
```

As you can see, standard Axum extractors like `Subject` are usable within the Controller methods, and the Controller can use `self` to access Dependencies and complete the work it needs to do. Other methods can be added that share the same dependencies, organized around a common business domain or other focus.

Couple this with a Provider that can be used to inject the dependency:

```rust
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Controller> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> inject::Result<Arc<Controller>> {
        let users = i.get(&users::SERVICE).await?;
        let handler = i.get(&socket::HANDLER).await?;

        Ok(Arc::new(Controller { users, handler }))
    }
}
```

The route can then be initialized with an Init hook:

```rust
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        let events_controller = i.get(&events::CONTROLLER).await?;

        i.handle(routes::Init::new(
            Method::GET,
            "/events",
            move |sub, ws| async move {
                events::Controller::upgrade(events_controller, sub, ws).await
            },
        ))
        .await?;

        Ok(())
    }
}
```

The `move |sub, ws| async move {}` function is a necessary shim to wrap the method to use it as a handler.

### Functional Handlers

Functional handlers eschew the typical Nakago approach of using a struct with Dependencies on the `self` instance, and instead use async functions with access to an Axum State extractor that pulls the `Inject` container out of the State.

Here's an example of a route handler implemented as an async function that uses the Inject container to retrieve a Users Service and a WebSocket connection handler::

```rust
use nakago_axum::{auth::Subject, Error, Inject};

pub async fn upgrade(
    Inject(i): Inject,
    sub: Subject,
    ws: WebSocketUpgrade,
) -> axum::response::Result<impl IntoResponse> {
    let users = i.get(&users::SERVICE).await.map_err(Error)?;
    let handler = i.get(&socket::HANDLER).await.map_err(Error)?;

    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        users.get_by_username(username, &true).await.unwrap_or(None)
    } else {
        None
    };

    Ok(ws.on_upgrade(|socket| async move { handler.handle(socket, user).await }))
}
```

The `Inject` extractor from the `nakago_axum` package is used to retrieve the `Inject` container from the State. This container is then used to retrieve the `users::SERVICE` and `socket::HANDLER` services from the container, mapping the errors to the special `nakago_axum:Error` wrapper that works as an Axum response.

Then you can initialize the route in an Init hook:

```rust
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, _i: Inject) -> inject::Result<()> {
        i.handle(routes::Init::new(
            Method::GET,
            "/events",
            events::upgrade,
        ))
        .await?;

        Ok(())
    }
}
```

## Starting the Application

To start your application, pass in your top-level Config type and create an instance. Attach Hooks in the order that they should be executed:

```rust
let mut app = AxumApplication::<Config>::default();
app.on(&EventType::Load, authz::Load::default());
app.on(&EventType::Load, graphql::Load::default());
app.on(&EventType::Init, graphql::Init::default());
```

Then, use `run` to start the application and return the connection details.

```rust
let server = app.run(args.config_path).await?;
let addr = server.local_addr();

info!("Started on port: {port}", port = addr.port());

server.await?;
```

## Integration Testing

Testing is handled by initializing your application server in a way similar to Production, and using a Lazy OnceCell to hold an HttpConnector you can use to make requests to the application.

```rust
static HTTP_CLIENT: Lazy<Client<HttpsConnector<HttpConnector>>> = Lazy::new(http_client);

/// Creates an http/https client via Hyper
pub fn http_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder().build::<_, Body>(HttpsConnector::new())
}
```

This can then be used to make requests to the running application instance for integration testing:

```rust
let mut req = Request::builder().method(Method::POST).uri(&self.url);

let resp = http_client.request(req).await?;
```

See the [Async-GraphQL Example's integration tests](https://github.com/bkonkle/nakago/tree/feature/nakago-sea-orm/examples/async-graphql/tests) for examples of how to use this pattern. This will evolve as more pieces are moved into the framework itself over time.

### CI Integration Testing

This strategy can be used for integration testing in the CI service of your choice based on a Docker Compose formation of shallow dependencies. This allows you to set up things like LocalStack or Postgers within your CI Docker environment and run integration tests against them without needing to use deployed resources. Branch-specific PR's are easy to run tests for in isolation.

Stay tuned for more details on how to set up this approach in your CI environment.
