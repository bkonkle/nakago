use std::sync::Arc;

use async_trait::async_trait;
use nakago::{to_provider_error, Dependency, Hook, Inject, InjectResult, Provider};
use nakago_axum::auth::{
    providers::{AUTH_STATE, JWKS},
    ProvideAuthState, ProvideJwks,
};
use oso::PolarClass;

use crate::{
    config::AppConfig,
    db::providers::{ProvideDatabaseConnection, DATABASE_CONNECTION},
    domains::{
        episodes::{self, model::Episode},
        profiles::{self, model::Profile},
        providers::init_domains,
        shows::{self, model::Show},
        users::{self, model::User, providers::USERS_SERVICE},
    },
    events::{
        providers::{CONNECTIONS, SOCKET_HANDLER},
        ProvideConnections, ProvideSocket,
    },
    graphql::{InitGraphQLSchema, GRAPHQL_SCHEMA},
    handlers::{EventsState, GraphQLState},
    routes::AppState,
    utils::providers::{add_app_config_loaders, ProvideOso, OSO},
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
pub struct ProvideAppState {}

#[async_trait]
impl Provider for ProvideAppState {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let auth = i.get(&AUTH_STATE).await?;
        let users = i.get(&USERS_SERVICE).await?;
        let handler = i.get(&SOCKET_HANDLER).await?;
        let schema = i.get(&GRAPHQL_SCHEMA).await?;

        let events = EventsState::new(users.clone(), handler.clone());
        let graphql = GraphQLState::new(users, schema);

        Ok(Arc::new(AppState::new((*auth).clone(), events, graphql)))
    }
}

/// Initialize the Application
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
#[derive(Default)]
pub struct InitApp {}

#[async_trait]
impl Hook for InitApp {
    /// Initialize the ConfigLoaders needed for Axum integration. Injects `Tag(ConfigLoaders)` if it
    /// has not been provided yet.
    async fn handle(&self, i: &Inject) -> InjectResult<()> {
        add_app_config_loaders().handle(i).await?;

        Ok(())
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
    async fn handle(&self, i: &Inject) -> InjectResult<()> {
        i.provide(&JWKS, ProvideJwks::<AppConfig>::default())
            .await?;
        i.provide(&DATABASE_CONNECTION, ProvideDatabaseConnection::default())
            .await?;
        i.provide(&OSO, ProvideOso::default()).await?;
        i.provide(&CONNECTIONS, ProvideConnections::default())
            .await?;

        init_domains(i).await?;
        init_authz(i).await?;

        InitGraphQLSchema::default().handle(i).await?;

        i.provide(&SOCKET_HANDLER, ProvideSocket::default()).await?;
        i.provide(&AUTH_STATE, ProvideAuthState::default()).await?;

        i.provide_type::<AppState>(ProvideAppState::default())
            .await?;

        Ok(())
    }
}

/// Initialize the authorization system. Must be initialized before the InitGraphQLSchema hook.
///
/// **Depends on (and modifies):**
///   - `Tag(Oso)`
pub async fn init_authz(i: &Inject) -> InjectResult<()> {
    // Set up authorization
    let mut oso = (*i.get(&OSO).await?).clone();

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

    i.replace(&OSO, oso).await?;

    Ok(())
}
