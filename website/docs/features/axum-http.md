---
sidebar_position: 3
---

# Axum HTTP Applications

The `nakago-axum` crate defines `AxumApplication`, which wraps `Application` and provides a way to Run an HTTP service and use the `Inject` container via Axum's `State` mechanism.

## Application Lifecycle

### Init

The `Init` Hook for an Axum Application automatically adds the `http::Config` and `auth::Config` Loaders before the user-provided hook is invoked and the Config is generated.

### Startup

The `Startup` Hook for an Axum Application uses the State with the provided Router, allowing the flow of the application to proceed through the Axum request handlers.

## Routes

Routes are initialized on Init. They have access to an Axum State extractor that pulls the `Inject` container in to be used within handlers. This approach is similar to Async-GraphQL's [data facility](https://async-graphql.github.io/async-graphql/en/context.html#store-data).

### Handlers

Here's an example of a route handler that needs access to the Inject container to retrieve a Users Service and a WebSocket connection handler::

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

### Routing

Then, to init a Route you pass the HTTP method, path, and handler to the `routes::Init` Hook:

```rust
app.on(&EventType::Init, routes::Init::new(Method::GET, "/events", events::upgrade));
```

This merges the route into the top-level Axum route for the application, mapping requests on `/events` with a GET method to the `events::upgrade` handler.

## Starting the Application

To start your application, pass in your top-level Config type and create an instance. Attach Hooks in the order that they should be executed:

```rust
let mut app = AxumApplication::<Config>::default();
app.on(&EventType::Load, users::schema::Load::default());
app.on(&EventType::Load, authz::Load::default());
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
