use async_trait::async_trait;
use nakago::{Inject, InjectResult, Provide, Tag};
use oso::Oso;

/// The Oso Tag
pub const OSO: Tag<Oso> = Tag::new("Oso");

/// Provide an Oso authorization instance
///
/// **Provides:** `Oso`
#[derive(Default)]
pub struct ProvideOso {}

#[async_trait]
impl Provide<Oso> for ProvideOso {
    async fn provide(&self, _i: &Inject) -> InjectResult<Oso> {
        Ok(Oso::new())
    }
}
