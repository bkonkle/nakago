use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};
use nakago_axum::auth::{
    providers::{AUTH_STATE, JWKS},
    ProvideAuthState, ProvideJwks,
};

use crate::{
    config::AppConfig,
    db::providers::{ProvideDatabaseConnection, DATABASE_CONNECTION},
    events::{
        providers::{CONNECTIONS, SOCKET_HANDLER},
        ProvideConnections, ProvideSocket,
    },
    graphql::{ProvideGraphQLSchema, GRAPHQL_SCHEMA},
    routes::{init_events_route, init_graphql_route, init_health_route, AppState, ProvideAppState},
    utils::{
        authz::{ProvideOso, OSO},
        config::init_config_loaders,
    },
};

/// Initializes dependency Providers for the Application
#[derive(Default)]
pub struct InitApp {}

#[async_trait]
impl Hook for InitApp {
    async fn handle(&self, i: &Inject) -> InjectResult<()> {
        // First add some final providers
        i.provide(&JWKS, ProvideJwks::<AppConfig>::default())
            .await?;
        i.provide(&DATABASE_CONNECTION, ProvideDatabaseConnection::default())
            .await?;
        i.provide(&OSO, ProvideOso::default()).await?;
        i.provide(&CONNECTIONS, ProvideConnections::default())
            .await?;

        i.provide(&SOCKET_HANDLER, ProvideSocket::default()).await?;
        i.provide(&AUTH_STATE, ProvideAuthState::default()).await?;
        i.provide(&GRAPHQL_SCHEMA, ProvideGraphQLSchema::default())
            .await?;

        i.provide_type::<AppState>(ProvideAppState::default())
            .await?;

        // Then, eagerly run some dependent init hooks
        init_config_loaders().handle(i).await?;
        init_health_route().handle(i).await?;
        init_graphql_route().handle(i).await?;
        init_events_route().handle(i).await?;

        Ok(())
    }
}
