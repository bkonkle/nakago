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
    routes::{AppState, ProvideAppState},
    utils::authz::{ProvideOso, OSO},
};

/// Initializes dependency Providers for the Application
#[derive(Default)]
pub struct InitApp {}

#[async_trait]
impl Hook for InitApp {
    async fn handle(&self, i: &Inject) -> InjectResult<()> {
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

        Ok(())
    }
}
