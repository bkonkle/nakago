use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use super::{
    episodes::schema::LoadEpisodes, profiles::schema::LoadProfiles,
    role_grants::schema::LoadRoleGrants, shows::schema::LoadShows, users::schema::LoadUsers,
};

/// Load dependencies needed for the domains
#[derive(Default)]
pub struct LoadDomains {}

#[async_trait]
impl Hook for LoadDomains {
    async fn handle(&self, i: Inject) -> InjectResult<()> {
        i.handle(LoadUsers::default()).await?;

        i.handle(LoadRoleGrants::default()).await?;

        i.handle(LoadProfiles::default()).await?;

        i.handle(LoadShows::default()).await?;

        i.handle(LoadEpisodes::default()).await?;

        Ok(())
    }
}
