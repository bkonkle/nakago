use async_trait::async_trait;
use nakago::{to_provider_error, Hook, Inject, InjectResult, Provide};
use nakago_axum::auth::{jwks::JWKS, state::AUTH_STATE, ProvideAuthState, ProvideJwks};
use oso::PolarClass;

use crate::{
    config::AppConfig,
    db::provider::{DatabaseConnectionProvider, DATABASE_CONNECTION},
    domains::{
        episodes::{self, model::Episode},
        profiles::{self, model::Profile},
        shows::{self, model::Show},
        users::{self, model::User, USERS_SERVICE},
        StartDomains,
    },
    events::{ConnectionsProvider, SocketProvider, CONNECTIONS, SOCKET_HANDLER},
    graphql::{InitGraphQLSchema, GRAPHQL_SCHEMA},
    handlers::{EventsState, GraphQLState},
    routes::AppState,
    utils::authz::{ProvideOso, OSO},
};

/// Provide the AppState for Axum
///
/// **Provides:** `AppState`
///
/// **Depends on:**
///   - `Tag(AuthState)`
///   - `Tag(UsersService)`
///   - `Tag(SocketHandler)`
#[derive(Default)]
pub struct Provider {}

#[async_trait]
impl Provide<AppState> for Provider {
    async fn provide(&self, i: &Inject) -> InjectResult<AppState> {
        let auth = i.get(&AUTH_STATE)?;
        let users = i.get(&USERS_SERVICE)?;
        let handler = i.get(&SOCKET_HANDLER)?;
        let schema = i.get(&GRAPHQL_SCHEMA)?;

        let events = EventsState::new(users, handler.clone());
        let graphql = GraphQLState::new(users, schema.clone());

        Ok(AppState::new(auth.clone(), events, graphql))
    }
}

/// Prepare to start the Application
///
/// **Provides:**:
///   - `Tag(JWKS)`
///   - `Tag(DatabaseConnection)`
///   - `Tag(Oso)`
///   - `Tag(Connections)`
///   - `Tag(GraphQLSchema)`
///   - `Tag(SocketHandler)`
///   - `Tag(AuthState)`
///   - `AppState`
#[derive(Default)]
pub struct StartApp {}

#[async_trait]
impl Hook for StartApp {
    async fn handle(&self, i: &mut Inject) -> InjectResult<()> {
        i.provide(&JWKS, ProvideJwks::<AppConfig>::default())
            .await?;
        i.provide(&DATABASE_CONNECTION, DatabaseConnectionProvider::default())
            .await?;
        i.provide(&OSO, ProvideOso::default()).await?;
        i.provide(&CONNECTIONS, ConnectionsProvider::default())
            .await?;

        i.handle(StartDomains::default()).await?;

        init_authz(i).await?;

        InitGraphQLSchema::default().handle(i).await?;

        i.provide(&SOCKET_HANDLER, SocketProvider::default())
            .await?;
        i.provide(&AUTH_STATE, ProvideAuthState::default()).await?;

        i.provide_type(Provider::default()).await?;

        Ok(())
    }
}

/// Initialize the authorization system. Must be initialized before the InitGraphQLSchema hook.
///
/// **Depends on (and modifies):**
///   - `Tag(Oso)`
pub async fn init_authz(i: &mut Inject) -> InjectResult<()> {
    // Set up authorization
    let oso = i.get_mut(&OSO)?;

    oso.register_class(User::get_polar_class_builder().name("User").build())
        .map_err(to_provider_error)?;
    oso.register_class(Profile::get_polar_class_builder().name("Profile").build())
        .map_err(to_provider_error)?;
    oso.register_class(Show::get_polar_class_builder().name("Show").build())
        .map_err(to_provider_error)?;
    oso.register_class(Episode::get_polar_class_builder().name("Episode").build())
        .map_err(to_provider_error)?;

    oso.load_str(
        &[
            users::AUTHORIZATION,
            profiles::AUTHORIZATION,
            shows::AUTHORIZATION,
            episodes::AUTHORIZATION,
        ]
        .join("\n"),
    )
    .map_err(to_provider_error)?;

    Ok(())
}
