use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{config::loader::ConfigData, inject, Tag};
use std::{marker::PhantomData, sync::Arc};

use crate::config::HttpConfig;

use super::{authenticate::AuthState, jwks};

/// The AuthConfig Tag
// pub const AUTH_CONFIG: Tag<Box<dyn FromRef<AuthConfig>>> = Tag::new("AuthConfig");

/// The JWKS Tag
pub const JWKS: Tag<Arc<jwks::JWKS>> = Tag::new("JWKS");

/// Provide the Json Web Key Set
///
/// **Provides:** `Arc<jwks::JWKS>`
///
/// **Depends on:**
///   - `<C: ConfigData>` - requires that `C` fulfills the `HttpConfig: FromRef<C>` constraint
#[derive(Default)]
pub struct ProvideJwks<C: ConfigData> {
    _phantom: PhantomData<C>,
}

#[async_trait]
impl<C: ConfigData> inject::Provider<jwks::JWKS> for ProvideJwks<C>
where
    HttpConfig: FromRef<C>,
{
    async fn provide(&self, i: &inject::Inject) -> inject::Result<jwks::JWKS> {
        let config = i.get::<C>()?;
        let http = HttpConfig::from_ref(config);
        let key_set = jwks::init(http.auth).await;

        Ok(key_set)
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
        let jwks = i.get_tag(&JWKS)?;
        let auth_state = AuthState::new(jwks.clone());

        Ok(auth_state)
    }
}
