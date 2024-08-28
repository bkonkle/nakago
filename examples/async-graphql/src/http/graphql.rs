use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{self, IntoResponse};
use nakago_axum::{auth::Subject, Inject};

use crate::domains::{graphql, users};

/// Handle GraphQL Requests
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

/// Handle GraphiQL UI Requests
pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}
