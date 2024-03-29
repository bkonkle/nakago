use std::{any::Any, collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use axum::extract::ws::Message;
use derive_new::new;
use nakago::{provider, Inject, Provider};
use nakago_derive::Provider;
use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use ulid::Ulid;

/// User Connection for WebSocket connections
#[derive(Debug, Clone, new)]
pub struct Connection<U> {
    tx: mpsc::UnboundedSender<Message>,

    #[allow(dead_code)]
    session: Session<U>,
}

impl<U> Connection<U> {
    /// Send a Message to the connection
    pub fn send(&self, message: Message) -> Result<()> {
        if let Err(_disconnected) = self.tx.send(message) {
            // The tx is disconnected
        }

        Ok(())
    }
}

/// The Connection for each currently connected User
///
/// - Key is their connection id
/// - Value is a sender of `axum::extract::ws::Message`
#[derive(Default, new)]
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
    pub async fn send(&self, conn_id: &str, message: Message) -> Result<()> {
        if let Some(connection) = self.0.read().await.get(conn_id) {
            return connection.send(message);
        }

        Err(anyhow!("Connection not found"))
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

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, new)]
    struct User {
        id: String,
    }

    #[tokio::test]
    async fn test_connection_send_success() -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let conn = Connection::new(tx, Session::<User>::Anonymous);

        conn.send(Message::Text("Hello, World!".to_string()))?;

        let message = rx.recv().await.ok_or(anyhow!("No message received"))?;

        assert_eq!(message, Message::Text("Hello, World!".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_connections_get_session_success() -> Result<()> {
        let connections = Connections::<User>::default();

        let expected = Session::<User>::User {
            user: User::new(Ulid::new().to_string()),
        };

        let conn_id = connections
            .insert(mpsc::unbounded_channel().0, expected.clone())
            .await;

        let session = connections.get_session(&conn_id).await;

        assert_eq!(expected, session);

        Ok(())
    }

    #[tokio::test]
    async fn test_connections_set_session_success() -> Result<()> {
        let connections = Connections::<User>::default();

        let expected = Session::<User>::User {
            user: User::new(Ulid::new().to_string()),
        };

        let conn_id = connections
            .insert(mpsc::unbounded_channel().0, Session::<User>::Anonymous)
            .await;

        connections.set_session(&conn_id, expected.clone()).await;

        let session = connections.get_session(&conn_id).await;

        assert_eq!(expected, session);

        Ok(())
    }

    #[tokio::test]
    async fn test_connections_send_success() -> Result<()> {
        let connections = Connections::<User>::default();

        let (tx, mut rx) = mpsc::unbounded_channel();
        let conn_id = connections.insert(tx, Session::<User>::Anonymous).await;

        connections
            .send(&conn_id, Message::Text("Hello, World!".to_string()))
            .await?;

        let message = rx.recv().await.ok_or(anyhow!("No message received"))?;

        assert_eq!(message, Message::Text("Hello, World!".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_connections_remove_success() -> Result<()> {
        let connections = Connections::<User>::default();

        let conn_id = connections
            .insert(
                mpsc::unbounded_channel().0,
                Session::<User>::User {
                    user: User::new(Ulid::new().to_string()),
                },
            )
            .await;

        connections.remove(&conn_id).await;

        let session = connections.get_session(&conn_id).await;

        assert_eq!(Session::<User>::Anonymous, session);

        Ok(())
    }

    #[tokio::test]
    async fn test_session_get_user_success() -> Result<()> {
        let user = User::new(Ulid::new().to_string());
        let session = Session::<User>::User { user: user.clone() };

        assert_eq!(Some(&user), session.get_user());

        Ok(())
    }
}
