use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{
    config::loader::Config,
    inject::{self, container::Dependency},
    Tag,
};
use std::{marker::PhantomData, sync::Arc};

use super::{authenticate::AuthState, config::AuthConfig, jwks};

/// The JWKS Tag
pub const JWKS: Tag<jwks::JWKS> = Tag::new("JWKS");

/// Provide the Json Web Key Set
///
/// **Provides:** `Arc<jwks::JWKS>`
///
/// **Depends on:**
///   - `<C: Config>` - requires that `C` fulfills the `AuthConfig: FromRef<C>` constraint
#[derive(Default)]
pub struct ProvideJwks<C: Config> {
    _phantom: PhantomData<C>,
}

#[async_trait]
impl<C: Config> inject::Provider for ProvideJwks<C>
where
    AuthConfig: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: inject::Inject) -> inject::Result<Arc<Dependency>> {
        let config = i.get_type::<C>().await?;
        let auth = AuthConfig::from_ref(&*config);
        let key_set = jwks::init(auth).await;

        Ok(Arc::new(key_set))
    }
}

/// The AuthState Tag
pub const AUTH_STATE: Tag<AuthState> = Tag::new("AuthState");

/// Provide the AuthState needed in order to use the `Subject` extractor in an Axum handler
///
/// **Provides:** `AuthState`
///
/// **Depends on:**
///   - `Tag(JWKS)`
#[derive(Default)]
pub struct ProvideAuthState {}

#[async_trait]
impl inject::Provider for ProvideAuthState {
    async fn provide(self: Arc<Self>, i: inject::Inject) -> inject::Result<Arc<Dependency>> {
        let jwks = i.get(&JWKS).await?;
        let auth_state = AuthState::new(jwks);

        Ok(Arc::new(auth_state))
    }
}
