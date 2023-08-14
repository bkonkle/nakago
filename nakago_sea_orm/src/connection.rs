use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{to_provider_error, Config, Dependency, Inject, InjectResult, Provider, Tag};
use sea_orm::DatabaseConnection;

use crate::config::DatabaseConfig;

/// Tag(SeaORM:DatabaseConnection)
pub const DATABASE_CONNECTION: Tag<DatabaseConnection> = Tag::new("SeaORM:DatabaseConnection");

/// Provide a SeaOrm Database connection
///
/// **Provides:** `Arc<DatabaseConnection>`
///
/// **Depends on:**
///   - `<C: Config>` - requires that `C` fulfills the `DatabaseConfig: FromRef<C>` constraint
#[derive(Default)]
pub struct ProvideConnection<C: Config> {
    _phantom: PhantomData<C>,
}

#[async_trait]
impl<C: Config> Provider for ProvideConnection<C>
where
    DatabaseConfig: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<Dependency>> {
        let config = i.get_type::<C>().await?;
        let database = DatabaseConfig::from_ref(&*config);

        Ok(Arc::new(
            sea_orm::Database::connect(&database.url)
                .await
                .map_err(to_provider_error)?,
        ))
    }
}
