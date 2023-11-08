use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{Html, IntoResponse};
use nakago_async_graphql::errors;
use nakago_axum::{auth::Subject, state};

use crate::{domains::users, graphql};

/// Handle GraphiQL Requests
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

/// Handle GraphQL Requests
pub async fn resolve(
    state::Inject(i): state::Inject,
    sub: Subject,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, GraphQLResponse> {
    let users = i
        .get(&users::SERVICE)
        .await
        .map_err(errors::to_graphql_response)?;

    let schema = i
        .get(&graphql::SCHEMA)
        .await
        .map_err(errors::to_graphql_response)?;

    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        users.get_by_username(username, &true).await.unwrap_or(None)
    } else {
        None
    };

    // Add the Subject and optional User to the context
    let request = req.into_inner().data(sub).data(user);

    Ok(schema.execute(request).await.into())
}
