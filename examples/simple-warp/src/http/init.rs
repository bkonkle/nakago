use async_trait::async_trait;
use nakago::{hooks, Hook, Inject};
use nakago_warp::routes;

use super::{health, user};

/// Init all handlers
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        // i.handle(routes::Init::new(health::health_check())).await?;

        // i.handle(routes::Init::new(user::get_username(i.clone())))
        //     .await?;

        i.handle(routes::Init::new(
            Method::GET,
            "/health",
            health::health_check,
        ))
        .await?;

        i.handle(routes::Init::new(
            Method::GET,
            "/username",
            user::get_username,
        ))
        .await?;

        Ok(())
    }
}
