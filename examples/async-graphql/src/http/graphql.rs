use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::response::{Html, IntoResponse};
use nakago::{provider, Inject, Provider};
use nakago_axum::auth::Subject;
use nakago_derive::Provider;

use crate::domains::{graphql, users};

/// Events Controller
#[derive(Clone)]
pub struct Controller {
    users: Arc<Box<dyn users::Service>>,
    schema: Arc<graphql::Schema>,
}

impl Controller {
    /// Handle GraphiQL Requests
    pub async fn graphiql() -> impl IntoResponse {
        Html(GraphiQLSource::build().endpoint("/graphql").finish())
    }

    /// Handle GraphQL Requests
    pub async fn resolve(
        self: Arc<Self>,
        sub: Subject,
        req: GraphQLRequest,
    ) -> Result<GraphQLResponse, GraphQLResponse> {
        // Retrieve the request User, if username is present
        let user = if let Subject(Some(ref username)) = sub {
            self.users
                .get_by_username(username, &true)
                .await
                .unwrap_or(None)
        } else {
            None
        };

        // Add the Subject and optional User to the context
        let request = req.into_inner().data(sub).data(user);

        Ok(self.schema.execute(request).await.into())
    }
}

/// Events Provider
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<Controller> for Provide {
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<Controller>> {
        let users = i.get::<Box<dyn users::Service>>().await?;
        let schema = i.get::<graphql::Schema>().await?;

        Ok(Arc::new(Controller { users, schema }))
    }
}
