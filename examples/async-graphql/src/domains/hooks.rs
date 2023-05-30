use async_trait::async_trait;
use nakago::{inject, Hook};

use super::{
    episodes::{EpisodeLoaderProvider, EpisodesServiceProvider, EPISODES_SERVICE, EPISODE_LOADER},
    profiles::{ProfileLoaderProvider, ProfilesServiceProvider, PROFILES_SERVICE, PROFILE_LOADER},
    role_grants::{
        RoleGrantLoaderProvider, RoleGrantsServiceProvider, ROLE_GRANTS_SERVICE, ROLE_GRANT_LOADER,
    },
    shows::{ShowLoaderProvider, ShowsServiceProvider, SHOWS_SERVICE, SHOW_LOADER},
    users::{UserLoaderProvider, UsersServiceProvider, USERS_SERVICE, USER_LOADER},
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
pub struct StartDomains {}

#[async_trait]
impl Hook for StartDomains {
    async fn handle(&self, i: &mut inject::Inject) -> inject::Result<()> {
        i.provide(&USERS_SERVICE, UsersServiceProvider::default())
            .await?;

        i.provide(&USER_LOADER, UserLoaderProvider::default())
            .await?;

        i.provide(&PROFILES_SERVICE, ProfilesServiceProvider::default())
            .await?;

        i.provide(&PROFILE_LOADER, ProfileLoaderProvider::default())
            .await?;

        i.provide(&ROLE_GRANTS_SERVICE, RoleGrantsServiceProvider::default())
            .await?;

        i.provide(&ROLE_GRANT_LOADER, RoleGrantLoaderProvider::default())
            .await?;

        i.provide(&SHOWS_SERVICE, ShowsServiceProvider::default())
            .await?;

        i.provide(&SHOW_LOADER, ShowLoaderProvider::default())
            .await?;

        i.provide(&EPISODES_SERVICE, EpisodesServiceProvider::default())
            .await?;

        i.provide(&EPISODE_LOADER, EpisodeLoaderProvider::default())
            .await?;

        Ok(())
    }
}
