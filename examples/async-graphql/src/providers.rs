use std::sync::Arc;

use async_trait::async_trait;
use nakago::{config::AddConfigLoaders, inject, Tag};
use nakago_axum::auth::{
    providers::{AUTH_STATE, JWKS},
    ProvideAuthState, ProvideJwks,
};
use oso::{Oso, PolarClass};

use crate::{
    config::DatabaseConfigLoader,
    db::providers::{ProvideDatabaseConnection, DATABASE_CONNECTION},
    domains::{
        episodes::{self, model::Episode},
        profiles::{self, model::Profile},
        providers::init_domains,
        shows::{self, model::Show},
        users::{self, model::User},
    },
    events::{
        providers::{CONNECTIONS, SOCKET_HANDLER},
        ProvideConnections, ProvideSocket,
    },
    graphql::InitGraphQLSchema,
    handlers::EventsState,
    router::AppState,
    AppConfig,
};

/// The Oso Tag
pub const OSO: Tag<Oso> = Tag::new("Oso");

/// Provide an Oso authorization instance
///
/// **Provides:** `Oso`
#[derive(Default)]
pub struct ProvideOso {}

#[async_trait]
impl inject::Provider<Oso> for ProvideOso {
    async fn provide(&self, _i: &inject::Inject) -> inject::Result<Oso> {
        Ok(Oso::new())
    }
}

/// Provide the AppState for Axum
///
/// **Provides:** `AppState`
///
/// **Depends on:**
///   - `Tag(AuthState)`
///   - `Tag(UserViewRepository)`
///   - `Tag(SocketHandler)`
#[derive(Default)]
pub struct ProvideAppState {}

#[async_trait]
impl inject::Provider<AppState> for ProvideAppState {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<AppState> {
        let auth = i.get(&AUTH_STATE)?;
        let handler = i.get(&SOCKET_HANDLER)?;

        let events = EventsState::new(handler.clone());

        Ok(AppState::new(auth.clone(), events))
    }
}

/// Initialize the Application
///
/// **Provides or Modifies:**
///   - `Tag(ConfigLoaders)`
#[derive(Default)]
pub struct InitApp {}

#[async_trait]
impl inject::Hook for InitApp {
    /// Initialize the ConfigLoaders needed for Axum integration. Injects `Tag(ConfigLoaders)` if it
    /// has not been provided yet.
    async fn handle(&self, i: &mut inject::Inject) -> inject::Result<()> {
        AddConfigLoaders::new(vec![Arc::<DatabaseConfigLoader>::default()])
            .handle(i)
            .await?;

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
///  - `Tag(GraphQLSchema)`
///   - `Tag(SocketHandler)`
///   - `Tag(AuthState)`
///   - `AppState`
#[derive(Default)]
pub struct StartApp {}

#[async_trait]
impl inject::Hook for StartApp {
    async fn handle(&self, i: &mut inject::Inject) -> inject::Result<()> {
        i.provide(&JWKS, ProvideJwks::<AppConfig>::default())
            .await?;
        i.provide(&DATABASE_CONNECTION, ProvideDatabaseConnection::default())
            .await?;
        i.provide(&OSO, ProvideOso::default()).await?;
        i.provide(&CONNECTIONS, ProvideConnections::default())
            .await?;

        init_domains(i).await?;

        InitGraphQLSchema::default().handle(i).await?;

        i.provide(&SOCKET_HANDLER, ProvideSocket::default()).await?;

        i.provide(&AUTH_STATE, ProvideAuthState::default()).await?;
        i.provide_type(ProvideAppState::default()).await?;

        init_authz(i).await
    }
}

/// Initialize the authorization system
pub async fn init_authz(i: &mut inject::Inject) -> inject::Result<()> {
    // Set up authorization
    let oso = i.get_mut(&OSO)?;

    oso.register_class(User::get_polar_class_builder().name("User").build())
        .map_err(inject::to_provider_error)?;
    oso.register_class(Profile::get_polar_class_builder().name("Profile").build())
        .map_err(inject::to_provider_error)?;
    oso.register_class(Show::get_polar_class_builder().name("Show").build())
        .map_err(inject::to_provider_error)?;
    oso.register_class(Episode::get_polar_class_builder().name("Episode").build())
        .map_err(inject::to_provider_error)?;

    oso.load_str(
        &[
            users::AUTHORIZATION,
            profiles::AUTHORIZATION,
            shows::AUTHORIZATION,
            episodes::AUTHORIZATION,
        ]
        .join("\n"),
    )
    .map_err(inject::to_provider_error)?;

    Ok(())
}
