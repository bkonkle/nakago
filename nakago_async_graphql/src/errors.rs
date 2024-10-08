use async_graphql::{Response, ServerError};
use async_graphql_axum::GraphQLResponse;

/// Convert a Nakago injection error into a GraphQL response
pub fn to_graphql_response(error: nakago::Error) -> GraphQLResponse {
    Response::from_errors(vec![ServerError::new(error.to_string(), None)]).into()
}
