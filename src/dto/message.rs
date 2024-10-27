use std::net::SocketAddr;

use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use bincode;

/// Represents the type of messages exchanged between nodes.
#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug, PartialEq)]
pub enum MessageType {
    Flooding,
    ChunksFound,
    ChunkRequest,
}

/// Represents a message exchanged between nodes.
#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub id: Option<String>,
    pub ttl: Option<u32>,
    pub filename: Option<String>,
    pub chunk_amount: u8,
    pub sender_ip: Option<SocketAddr>,
    pub sender_ip_tcp: Option<SocketAddr>,
    pub transfer_speed: Option<u32>,
    pub chunks: Option<Vec<u8>>,
}

impl Message {
    /// Creates a new flooding message.
    pub fn new_flooding(
        filename: String,
        sender_ip: SocketAddr,
        chunk_amount: u8,
        ttl: u32,
    ) -> Message {
        let id: String = OsRng
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        Message {
            message_type: MessageType::Flooding,
            id: Some(id),
            ttl: Some(ttl),
            filename: Some(filename),
            chunk_amount,
            sender_ip: Some(sender_ip),
            sender_ip_tcp: None,
            transfer_speed: None,
            chunks: None,
        }
    }

    /// Creates a new chunks found message.
    pub fn new_chunks_found(
        id: String,
        sender_ip_tcp: SocketAddr,
        chunk_amount: u8,
        chunks: &Vec<u8>,
        transfer_speed: u32,
    ) -> Message {
        Message {
            message_type: MessageType::ChunksFound,
            id: Some(id),
            ttl: None,
            filename: None,
            chunk_amount,
            sender_ip: None,
            sender_ip_tcp: Some(sender_ip_tcp),
            transfer_speed: Some(transfer_speed),
            chunks: Some(chunks.clone()),
        }
    }

    /// Creates a new chunk request message.
    pub fn new_chunk_request(filename: String, chunks: Vec<u8>) -> Message {
        Message {
            message_type: MessageType::ChunkRequest,
            id: None,
            ttl: None,
            filename: Some(filename),
            chunk_amount: 0,
            sender_ip: None,
            sender_ip_tcp: None,
            transfer_speed: None,
            chunks: Some(chunks),
        }
    }

    /// Decreases the TTL (Time To Live) of the message.
    pub fn decrease_ttl(&mut self) -> u32 {
        let new_ttl = self.ttl.unwrap_or(1).saturating_sub(1);
        self.ttl = Some(new_ttl);
        new_ttl
    }

    /// Serializes the message into bytes using `bincode::encode_to_vec`.
    pub fn get_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, bincode::config::standard())
            .expect("Failed to serialize message")
    }

    /// Deserializes bytes into a message using `bincode::decode_from_slice`.
    pub fn get_from_bytes(bytes: &[u8]) -> Message {
        let (decoded_message, _): (Message, usize) =
            bincode::decode_from_slice(bytes, bincode::config::standard())
                .expect("Failed to deserialize message");
        decoded_message
    }
}
