use std::sync::Arc;

use async_trait::async_trait;
use nakago::{to_provider_error, Dependency, Hook, Inject, InjectResult, Provider};
use nakago_axum::auth::providers::AUTH_STATE;
use oso::PolarClass;

use crate::{
    domains::{
        episodes::{self, model::Episode},
        profiles::{self, model::Profile},
        shows::{self, model::Show},
        users::{self, model::User, providers::USERS_SERVICE},
    },
    events::providers::SOCKET_HANDLER,
    graphql::GRAPHQL_SCHEMA,
    handlers::{EventsState, GraphQLState},
    routes::AppState,
    utils::providers::OSO,
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

/// Initialize the authorization system. Must be initialized before the InitGraphQLSchema hook.
///
/// **Depends on (and modifies):**
///   - `Tag(Oso)`
#[derive(Default)]
pub struct InitAuthz {}

#[async_trait]
impl Hook for InitAuthz {
    async fn handle(&self, i: &Inject) -> InjectResult<()> {
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
}
