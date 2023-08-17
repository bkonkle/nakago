use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{to_provider_error, Config, Inject, InjectResult, Provider, Tag};
use nakago_derive::Provider;
use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, MockDatabaseTrait};

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

#[Provider]
#[async_trait]
impl<C: Config> Provider<DatabaseConnection> for ProvideConnection<C>
where
    DatabaseConfig: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> InjectResult<Arc<DatabaseConnection>> {
        let config = i.get_type::<C>().await?;
        let database = DatabaseConfig::from_ref(&*config);

        Ok(Arc::new(
            sea_orm::Database::connect(&database.url)
                .await
                .map_err(to_provider_error)?,
        ))
    }
}

/// Provide a Mock Database Connection for use in unit testing
///
/// **Provides:** `Arc<DatabaseConnection>`
pub struct ProvideMockConnection {
    db: Mutex<MockDatabase>,
}

impl ProvideMockConnection {
    /// Initialize a new Mock DB connection
    pub fn new(db: MockDatabase) -> Self {
        Self { db: Mutex::new(db) }
    }

    /// Replace the MockDatabase instance inside the Provider, returning the original
    pub fn replace(&self, db: MockDatabase) -> MockDatabase {
        let mut existing = self.db.lock().expect("Could not lock MockDatabase Mutex");

        // Replace the current Mock Database with the given one
        std::mem::replace(&mut *existing, db)
    }
}

impl Default for ProvideMockConnection {
    fn default() -> Self {
        Self::new(MockDatabase::new(DatabaseBackend::Sqlite))
    }
}

#[Provider]
#[async_trait]
impl Provider<DatabaseConnection> for ProvideMockConnection {
    async fn provide(self: Arc<Self>, _i: Inject) -> InjectResult<Arc<DatabaseConnection>> {
        let existing = self.db.lock().expect("Could not lock MockDatabase Mutex");
        let backend = existing.get_database_backend();
        drop(existing);

        let db = self.replace(MockDatabase::new(backend));

        Ok(Arc::new(db.into_connection()))
    }
}
