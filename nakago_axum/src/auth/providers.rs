use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{config::loader::Config, inject, Tag};
use std::{marker::PhantomData, sync::Arc};

use super::{authenticate::AuthState, config::AuthConfig, jwks};

/// The AuthConfig Tag
// pub const AUTH_CONFIG: Tag<Box<dyn FromRef<AuthConfig>>> = Tag::new("AuthConfig");

/// The JWKS Tag
pub const JWKS: Tag<Arc<jwks::JWKS>> = Tag::new("JWKS");

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
impl<C: Config> inject::Provider<Arc<jwks::JWKS>> for ProvideJwks<C>
where
    AuthConfig: FromRef<C>,
{
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<jwks::JWKS>> {
        let config = i.get_type::<C>()?;
        let auth = AuthConfig::from_ref(config);
        println!(">- auth -> {:?}", auth);
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
pub struct ProvideAuthState {}

#[async_trait]
impl inject::Provider<AuthState> for ProvideAuthState {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<AuthState> {
        let jwks = i.get(&JWKS)?;
        let auth_state = AuthState::new(jwks.clone());

        Ok(auth_state)
    }
}
