use async_trait::async_trait;
use nakago::{inject, Tag};

use super::{authenticate::AuthState, jwks};

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
impl inject::Provide<AuthState> for ProvideAuthState {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<AuthState> {
        let jwks = i.get(&jwks::JWKS)?;
        let auth_state = AuthState::new(jwks.clone());

        Ok(auth_state)
    }
}
