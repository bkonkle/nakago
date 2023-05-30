use std::sync::Arc;

use async_trait::async_trait;
use nakago::{to_provider_error, Inject, InjectResult, Provide, Tag};
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
pub struct DatabaseConnectionProvider {}

#[async_trait]
impl Provide<Arc<DatabaseConnection>> for DatabaseConnectionProvider {
    async fn provide(&self, i: &Inject) -> InjectResult<Arc<DatabaseConnection>> {
        let config = i.get_type::<AppConfig>()?;

        Ok(Arc::new(
            sea_orm::Database::connect(&config.database.url)
                .await
                .map_err(to_provider_error)?,
        ))
    }
}
