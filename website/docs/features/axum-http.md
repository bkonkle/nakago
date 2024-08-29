---
sidebar_position: 3
---

# Using Nakago with Axum

The `nakago-axum` crate defines provides a way to easily use the Nakago `Inject` container via Axum's `State` mechanism.

## Axum State

Axum provides the State extractor that allows you to inject dependencies that stay the same across many requests. For DI-driven applications, however, your dependencies are provided through the injection container. Nakago's Axum helpers use State to automatically carry the injection container, but in a way that you don't have to think about while building typical applications.

Nakago provides an extractor called `Inject` that allows you to request dependencies from Nakago as smoothly as using any other Axum extractor. In this example, the "resolve" request handler uses `Inject` to request the `graphql::Schema` and a `users::Service` trait implementation from the injection container so that it can be used to handle the request. The `Subject` extractor is also used, to provide the JWT payload claims needed to find a User in the database:

```rust
use nakago_axum::{auth::Subject, Inject};

use crate::domains::{graphql, users};

pub async fn resolve(
    Inject(schema): Inject<graphql::Schema>,
    Inject(users): Inject<Box<dyn users::Service>>,
    sub: Subject,
    req: GraphQLRequest,
) -> GraphQLResponse {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        users.get_by_username(username, &true).await.unwrap_or(None)
    } else {
        None
    };

    // Add the Subject and optional User to the context
    let request = req.into_inner().data(sub).data(user);

    schema.execute(request).await.into()
}
```

Then you can initialize your top level Axum router in an initializer:

```rust
pub fn init(i: &Inject) -> Router {
    Router::new()
        .layer(trace_layer())
        .route("/health", get(health::health_check))
        .route("/graphql", get(graphql::graphiql).post(graphql::resolve))
        .route("/events", get(events::handle))
        .with_state(State::new(i.clone()))
}
```

## Integration Testing

Integration testing is handled by initializing your application server in a way similar to Production, using test utils to make requests to your server running in the background.

```rust
let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.test.toml".to_string());

let i = init::app(Some(config_path.clone().into())).await?;

i.replace_with::<Validator>(validator::ProvideUnverified::default())
    .await?;

let router = router::init(&i);

let utils = nakago_axum::test::Utils::init(i, "/", router).await?;

let username = Ulid::new().to_string();
let token = utils.create_jwt(&username).await?;

let resp = utils
    .http
    .request_json(Method::POST, "/username", Value::Null, Some(&token))
    .send()
    .await?;
```

See the [Async-GraphQL Example's integration tests](https://github.com/bkonkle/nakago/tree/main/examples/async-graphql/tests) for examples of how to use this pattern. This will evolve as more pieces are moved into the framework itself over time.

### CI Integration Testing

This strategy can be used for integration testing in the CI service of your choice based on a Docker Compose formation of shallow dependencies. This allows you to set up things like LocalStack or Postgers within your CI Docker environment and run integration tests against them without needing to use deployed resources. Branch-specific PR's are easy to run tests for in isolation.

Stay tuned for more details on how to set up this approach in your CI environment.
