use async_trait::async_trait;
use nakago::{inject, Hook, Inject};

use super::{health, user};

///
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        i.provide(&health::CHECK_ROUTE, health::ProvideCheck::default())
            .await?;

        i.provide(
            &user::GET_USERNAME_ROUTE,
            user::ProvideGetUsername::default(),
        )
        .await?;

        Ok(())
    }
}
