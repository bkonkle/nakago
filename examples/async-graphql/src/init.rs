use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};
use nakago_async_graphql::schema::{InitSchema, SchemaBuilderProvider};
use nakago_axum::auth::{
    ProvideAuthState, ProvideJwks, {AUTH_STATE, JWKS},
};
use nakago_sea_orm::{ProvideConnection, DATABASE_CONNECTION};

use crate::{
    config::AppConfig,
    events::{
        ProvideConnections, ProvideSocket, {CONNECTIONS, SOCKET_HANDLER},
    },
    graphql::{GRAPHQL_SCHEMA, GRAPHQL_SCHEMA_BUILDER},
    routes::{init_events_route, init_graphql_route, init_health_route, AppState, ProvideAppState},
    utils::authz::{ProvideOso, OSO},
};

/// Initializes dependency Providers for the Application
#[derive(Default)]
pub struct InitApp {}

#[async_trait]
impl Hook for InitApp {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        // First add some final providers
        i.provide(&JWKS, ProvideJwks::<AppConfig>::default())
            .await?;

        i.provide(
            &DATABASE_CONNECTION,
            ProvideConnection::<AppConfig>::default(),
        )
        .await?;

        i.provide(&OSO, ProvideOso::default()).await?;

        i.provide(&CONNECTIONS, ProvideConnections::default())
            .await?;

        i.provide(&SOCKET_HANDLER, ProvideSocket::default()).await?;

        i.provide(&AUTH_STATE, ProvideAuthState::default()).await?;

        i.provide(&GRAPHQL_SCHEMA_BUILDER, SchemaBuilderProvider::default())
            .await?;

        i.provide_type::<AppState>(ProvideAppState::default())
            .await?;

        // Then, immediately execute some dependent init hooks
        nakago_sea_orm::init_config_loaders()
            .handle(i.clone())
            .await?;

        // Initialize the GraphQL Schema
        InitSchema::default()
            .with_builder_tag(&GRAPHQL_SCHEMA_BUILDER)
            .with_schema_tag(&GRAPHQL_SCHEMA)
            .handle(i.clone())
            .await?;

        init_health_route().handle(i.clone()).await?;
        init_graphql_route().handle(i.clone()).await?;
        init_events_route().handle(i.clone()).await?;

        Ok(())
    }
}
