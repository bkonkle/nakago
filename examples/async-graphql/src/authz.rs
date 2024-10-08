use std::sync::Arc;

use async_trait::async_trait;
use nakago::{provider, to_provider_error, Inject, Provider};
use nakago_derive::Provider;
use oso::Oso;
use oso::PolarClass;

use crate::domains::{
    episodes::{self, model::Episode},
    profiles::{self, model::Profile},
    shows::{self, model::Show},
    users::{self, model::User},
};

/// Provide an Oso authorization instance
#[derive(Default)]
pub struct ProvideOso {}

#[Provider]
#[async_trait]
impl Provider<Oso> for ProvideOso {
    async fn provide(self: Arc<Self>, _i: Inject) -> provider::Result<Arc<Oso>> {
        Ok(Arc::new(Oso::new()))
    }
}

/// Load the authorization system. Must be invoked before the GraphQL Schema is initialized.
pub async fn load(i: &Inject) -> nakago::Result<()> {
    // Set up authorization
    let oso = i.get::<Oso>().await?;
    let mut oso = (*oso).clone();

    oso.register_class(User::get_polar_class_builder().name("User").build())
        .map_err(to_provider_error)?;
    oso.register_class(Profile::get_polar_class_builder().name("Profile").build())
        .map_err(to_provider_error)?;
    oso.register_class(Show::get_polar_class_builder().name("Show").build())
        .map_err(to_provider_error)?;
    oso.register_class(Episode::get_polar_class_builder().name("Episode").build())
        .map_err(to_provider_error)?;

    oso.load_str(
        &[
            users::AUTHORIZATION,
            profiles::AUTHORIZATION,
            shows::AUTHORIZATION,
            episodes::AUTHORIZATION,
        ]
        .join("\n"),
    )
    .map_err(to_provider_error)?;

    i.replace::<Oso>(oso).await?;

    Ok(())
}
