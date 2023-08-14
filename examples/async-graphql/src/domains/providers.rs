use async_trait::async_trait;
use nakago::{Hook, Inject, InjectResult};

use super::{
    episodes::providers::{
        ProvideEpisodeLoader, ProvideEpisodesService, EPISODES_SERVICE, EPISODE_LOADER,
    },
    profiles::providers::{
        ProvideProfileLoader, ProvideProfilesService, PROFILES_SERVICE, PROFILE_LOADER,
    },
    role_grants::providers::{
        ProvideRoleGrantLoader, ProvideRoleGrantsService, ROLE_GRANTS_SERVICE, ROLE_GRANT_LOADER,
    },
    shows::providers::{ProvideShowLoader, ProvideShowsService, SHOWS_SERVICE, SHOW_LOADER},
    users::providers::{ProvideUserLoader, ProvideUsersService, USERS_SERVICE, USER_LOADER},
};

/// Initialize the default View and CQRS providers for the domains
///
/// **Provides:**
///  - Tag(UsersService)
///  - Tag(UserLoader)
///  - Tag(ProfilesService)
///  - Tag(ProfileLoader)
///  - Tag(RoleGrantsService)
///  - Tag(RoleGrantLoader)
///  - Tag(ShowsService)
///  - Tag(ShowLoader)
///  - Tag(EpisodesService)
///  - Tag(EpisodeLoader)
///
/// **Depends on:**
///  - `Tag(DatabaseConnection)`
#[derive(Default)]
pub struct InitDomains {}

#[async_trait]
impl Hook for InitDomains {
    async fn handle(&self, i: &Inject) -> InjectResult<()> {
        i.provide(&USERS_SERVICE, ProvideUsersService::default())
            .await?;

        i.provide(&USER_LOADER, ProvideUserLoader::default())
            .await?;

        i.provide(&PROFILES_SERVICE, ProvideProfilesService::default())
            .await?;

        i.provide(&PROFILE_LOADER, ProvideProfileLoader::default())
            .await?;

        i.provide(&ROLE_GRANTS_SERVICE, ProvideRoleGrantsService::default())
            .await?;

        i.provide(&ROLE_GRANT_LOADER, ProvideRoleGrantLoader::default())
            .await?;

        i.provide(&SHOWS_SERVICE, ProvideShowsService::default())
            .await?;

        i.provide(&SHOW_LOADER, ProvideShowLoader::default())
            .await?;

        i.provide(&EPISODES_SERVICE, ProvideEpisodesService::default())
            .await?;

        i.provide(&EPISODE_LOADER, ProvideEpisodeLoader::default())
            .await?;

        Ok(())
    }
}
