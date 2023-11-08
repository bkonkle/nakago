use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use nakago::{inject, Inject, Provider};
use nakago_axum::{auth::Subject, Route};
use nakago_derive::Provider;
use tokio::sync::Mutex;

// TODO: Remove
// /// State for the GraphQL Handler
// #[derive(Clone)]
// pub struct State {
//     users: Arc<Box<dyn users::Service>>,
//     schema: Arc<graphql::Schema>,
// }

/// Handle GraphiQL Requests
pub async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

/// Handle GraphQL Requests
pub async fn graphql_handler(sub: Subject, req: GraphQLRequest) -> GraphQLResponse {
    // Retrieve the request User, if username is present
    let user = if let Subject(Some(ref username)) = sub {
        state
            .users
            .get_by_username(username, &true)
            .await
            .unwrap_or(None)
    } else {
        None
    };

    // Add the Subject and optional User to the context
    let request = req.into_inner().data(sub).data(user);

    state.schema.execute(request).await.into()
}

/// A Provider for the health check route
#[derive(Default)]
pub struct ProvideCheck {}

#[Provider]
#[async_trait]
impl Provider<Route> for ProvideCheck {
    async fn provide(self: Arc<Self>, _: Inject) -> inject::Result<Arc<Route>> {
        let route = Router::new().route("/graphql", get(graphiql).post(graphql_handler));

        Ok(Arc::new(Mutex::new(route)))
    }
}
