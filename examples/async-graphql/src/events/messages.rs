use axum::extract::ws::Message;
use fake::Dummy;
use serde::{Deserialize, Serialize};

/// Incoming `WebSocket` messages from clients
#[derive(Clone, Debug, Dummy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum IncomingMessage {
    /// A Ping message, which should echo back a Pong
    Ping,

    /// An indication that the message couldn't be serialized
    CannotDeserialize,
}

impl From<Message> for IncomingMessage {
    fn from(msg: Message) -> IncomingMessage {
        // IncomingMessage::from_message(msg).expect("Unable to deserialize IncomingMessage")
        let msg = if let Ok(message) = msg.to_text() {
            message
        } else {
            return IncomingMessage::CannotDeserialize;
        };

        serde_json::from_str(msg).unwrap_or(IncomingMessage::CannotDeserialize)
    }
}

/// Outgoing `WebSocket` JSON-friendly messages to clients
#[derive(Clone, Debug, Dummy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum OutgoingMessage {
    /// A Pong message, which is the response to a Ping
    Pong,

    /// An Error Response
    Error {
        /// The Error message
        message: String,
    },
}

impl From<OutgoingMessage> for Message {
    fn from(msg: OutgoingMessage) -> Message {
        Message::Text(serde_json::to_string(&msg).expect("Unable to serialize OutgoingMessage"))
    }
}
