use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use nakago::{self, provider, to_provider_error, utils::FromRef, Inject, Provider, Tag};
use nakago_derive::Provider;
use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase, MockDatabaseTrait};

use crate::Config;

/// Tag(nakago_sea_orm::DatabaseConnection)
pub const CONNECTION: Tag<DatabaseConnection> = Tag::new("nakago_sea_orm::DatabaseConnection");

/// Provide a SeaOrm Database connection
///
/// **Provides:** `Arc<nakago_sea_orm::DatabaseConnection>`
///
/// **Depends on:**
///   - `<Config>` - requires that `C` fulfills the `Config: FromRef<C>` constraint
#[derive(Default)]
pub struct Provide<C: nakago::Config> {
    config_tag: Option<&'static Tag<C>>,
    _phantom: PhantomData<C>,
}

impl<C: nakago::Config> Provide<C> {
    /// Create a new instance of Provide
    pub fn new() -> Self {
        Self {
            config_tag: None,
            ..Default::default()
        }
    }

    /// Set the config Tag for this instance
    pub fn with_config_tag(self, config_tag: &'static Tag<C>) -> Self {
        Self {
            config_tag: Some(config_tag),
            ..self
        }
    }
}

#[Provider]
#[async_trait]
impl<C: nakago::Config> Provider<DatabaseConnection> for Provide<C>
where
    Config: FromRef<C>,
{
    async fn provide(self: Arc<Self>, i: Inject) -> provider::Result<Arc<DatabaseConnection>> {
        let dep = if let Some(tag) = self.config_tag {
            i.get_tag(tag).await?
        } else {
            i.get::<C>().await?
        };

        let config = Config::from_ref(&*dep);

        Ok(Arc::new(
            sea_orm::Database::connect(&config.url)
                .await
                .map_err(to_provider_error)?,
        ))
    }
}

/// Provide a Mock Database Connection for use in unit testing
///
/// **Provides:** `Arc<nakago_sea_orm::DatabaseConnection>`
pub struct ProvideMock {
    db: Mutex<MockDatabase>,
}

impl ProvideMock {
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

impl Default for ProvideMock {
    fn default() -> Self {
        Self::new(MockDatabase::new(DatabaseBackend::Sqlite))
    }
}

#[Provider]
#[async_trait]
impl Provider<DatabaseConnection> for ProvideMock {
    async fn provide(self: Arc<Self>, _i: Inject) -> provider::Result<Arc<DatabaseConnection>> {
        let existing = self.db.lock().expect("Could not lock MockDatabase Mutex");
        let backend = existing.get_database_backend();
        drop(existing);

        let db = self.replace(MockDatabase::new(backend));

        Ok(Arc::new(db.into_connection()))
    }
}
