use std::sync::Arc;

use async_trait::async_trait;
use nakago::{inject, Tag};
use sea_orm::DatabaseConnection;

use crate::config::AppConfig;

/// The PostgresPool Tag
pub const DATABASE_CONNECTION: Tag<Arc<DatabaseConnection>> = Tag::new("DatabaseConnection");

/// Provide a SeaOrm Database connection
///
/// **Provides:** `Arc<DatabaseConnection>`
///
/// **Depends on:**
///   - `AppConfig`
#[derive(Default)]
pub struct ProvideDatabaseConnection {}

#[async_trait]
impl inject::Provider<Arc<DatabaseConnection>> for ProvideDatabaseConnection {
    async fn provide(&self, i: &inject::Inject) -> inject::Result<Arc<DatabaseConnection>> {
        let config = i.get_type::<AppConfig>()?;

        Ok(Arc::new(
            sea_orm::Database::connect(&config.database.url)
                .await
                .map_err(inject::to_provider_error)?,
        ))
    }
}
