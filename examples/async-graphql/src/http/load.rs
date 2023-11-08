use async_trait::async_trait;
use nakago::{inject, Hook, Inject};

use super::{events, graphql, health};

///
#[derive(Default)]
pub struct Load {}

#[async_trait]
impl Hook for Load {
    async fn handle(&self, i: Inject) -> inject::Result<()> {
        i.provide(&health::CHECK_ROUTE, health::ProvideCheck::default())
            .await?;

        i.provide(&graphql::RESOLVE_ROUTE, graphql::ProvideResolve::default())
            .await?;

        i.provide(&events::UPGRADE_ROUTE, events::ProvideUpgrade::default())
            .await?;

        Ok(())
    }
}
