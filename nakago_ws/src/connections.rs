use std::{any::Any, collections::HashMap, sync::Arc};

use async_trait::async_trait;
use axum::extract::ws::Message;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;
use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use ulid::Ulid;

/// User Connection for WebSocket connections
pub struct Connection<U> {
    tx: mpsc::UnboundedSender<Message>,

    #[allow(dead_code)]
    session: Session<U>,
}

/// The Connection for each currently connected User
///
/// - Key is their connection id
/// - Value is a sender of `axum::extract::ws::Message`
#[derive(Default)]
pub struct Connections<U>(Arc<RwLock<HashMap<String, Connection<U>>>>);

impl<U: Clone> Connections<U> {
    /// Get a copy of the Session associated with the given connection ID
    #[allow(dead_code)]
    pub async fn get_session(&self, conn_id: &str) -> Session<U> {
        self.0
            .write()
            .await
            .get(conn_id)
            .map(|conn| conn.session.clone())
            .unwrap_or_default()
    }

    /// Set the Session associated with the given connection ID, if it exists
    #[allow(dead_code)]
    pub async fn set_session(&self, conn_id: &str, session: Session<U>) {
        if let Some(connection) = self.0.write().await.get_mut(conn_id) {
            connection.session = session;
        }
    }

    /// Send a Message to the given connection at the given id
    pub async fn send(&self, conn_id: &str, message: Message) {
        if let Some(connection) = self.0.read().await.get(conn_id) {
            if let Err(_disconnected) = connection.tx.send(message) {
                // The tx is disconnected
            }
        }
    }

    ///. Inserts a connection into the hash map, and returns the id
    pub async fn insert(&self, tx: UnboundedSender<Message>, session: Session<U>) -> String {
        let conn_id = Ulid::new().to_string();

        self.0
            .write()
            .await
            .insert(conn_id.clone(), Connection { tx, session });

        conn_id
    }

    /// Removees a connection from the hash map
    pub async fn remove(&self, conn_id: &str) {
        self.0.write().await.remove(conn_id);
    }
}

/// A Session tracking details about this particular connection
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Session<U> {
    /// A session that is not associated with a User
    #[default]
    Anonymous,

    /// A session that is associated with a User
    User {
        /// The User instance
        user: U,
    },
}

impl<U> Session<U> {
    /// Create a new session for the given User
    pub fn new(user: Option<U>) -> Self {
        match user {
            Some(user) => Self::User { user },
            None => Self::Anonymous,
        }
    }

    /// Get the User associated with this session, if any
    #[allow(dead_code)]
    pub fn get_user(&self) -> Option<&U> {
        match self {
            Session::Anonymous => None,
            Session::User { user, .. } => Some(user),
        }
    }
}

/// Provide the default Connections implementation
#[derive(Default)]
pub struct Provide<U> {
    _phantom: std::marker::PhantomData<U>,
}

#[Provider]
#[async_trait]
impl<U: Send + Sync + Any + Default> Provider<Connections<U>> for Provide<U> {
    async fn provide(self: Arc<Self>, _i: Inject) -> provider::Result<Arc<Connections<U>>> {
        Ok(Arc::new(Connections::default()))
    }
}
