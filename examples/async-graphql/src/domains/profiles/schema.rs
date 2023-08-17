use async_graphql::{dataloader::DataLoader, EmptySubscription, Schema};
use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provider, Tag};
use std::sync::Arc;

use crate::domains::{
    profiles::{
        resolver::{ProfilesMutation, ProfilesQuery},
        service::PROFILES_SERVICE,
    },
    users::loaders::USER_LOADER,
};

/// Tag(ProfilesSchema)
#[allow(dead_code)]
pub const PROFILES_SCHEMA: Tag<Box<ProfilesSchema>> = Tag::new("ProfilesSchema");

/// The ProfilesSchema, covering just the Profiles domain
pub type ProfilesSchema = Schema<ProfilesQuery, ProfilesMutation, EmptySubscription>;

/// Provide the ProfilesSchema
///
/// **Provides:** `Arc<ProfilesSchema>`
///
/// **Depends on:**
///   - `Tag(ProfilesService)`
///   - `Tag(ShowLoader)`
#[derive(Default)]
pub struct ProvideProfilesSchema {}

#[async_trait]
impl Provider<ProfilesSchema> for ProvideProfilesSchema {
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<ProfilesSchema>> {
        let service = i.get(&PROFILES_SERVICE).await?;
        let user_loader = i.get(&USER_LOADER).await?;

        let schema: ProfilesSchema = Schema::build(
            ProfilesQuery::default(),
            ProfilesMutation::default(),
            EmptySubscription,
        )
        .data(service)
        .data(DataLoader::new(user_loader, tokio::spawn))
        .finish();

        Ok(Arc::new(schema))
    }
}
