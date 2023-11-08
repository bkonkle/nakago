use async_graphql::ServerError;
use async_graphql_axum::GraphQLResponse;

/// Convert a Nakago injection error into a GraphQL response
pub fn to_graphql_response(error: nakago::inject::Error) -> GraphQLResponse {
    async_graphql::Response::from_errors(vec![ServerError::new(error.to_string(), None)]).into()
}
