/// Tag(graphql::Controller)
pub const CONTROLLER: Tag<Controller> = Tag::new("graphql::Controller");

/// State for the GraphQL Handler
#[derive(Clone)]
pub struct Controller {
    users: Arc<Box<dyn users::Service>>,
    schema: Arc<graphql::Schema>,
}

impl Controller {
    /// Create a new GraphQLState instance
    pub fn new(users: Arc<Box<dyn users::Service>>, schema: Arc<graphql::Schema>) -> Self {
        Self { users, schema }
    }

    /// Handle GraphiQL Requests
    pub async fn graphiql(self) -> impl IntoResponse {
        Html(GraphiQLSource::build().endpoint("/graphql").finish())
    }

    /// Handle GraphQL Requests
    pub async fn graphql_handler(self, sub: Subject, req: GraphQLRequest) -> GraphQLResponse {
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

        self.schema.execute(request).await.into()
    }
}
