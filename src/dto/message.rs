use std::net::SocketAddr;

use rand::{distributions::Alphanumeric, Rng};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug, PartialEq)]
pub enum MessageType {
    Flooding,
    ChunksFound
}

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
    pub fn new_flooding(filename: String, sender_ip: SocketAddr, chunk_amount: u8, ttl: u32) -> Message {
        let id: String = rand::thread_rng()
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
            chunks: None
        }
    }

    pub fn new_chunks_found(id: String, sender_ip_tcp: SocketAddr, chunk_amount: u8, chunks: &Vec<u8>, transfer_speed: u32) -> Message {

        Message {
            message_type: MessageType::ChunksFound,
            id: Some(id), 
            ttl: None, 
            filename: None, 
            chunk_amount,
            sender_ip: None,
            sender_ip_tcp: Some(sender_ip_tcp),
            transfer_speed: Some(transfer_speed),
            chunks: Some(chunks.to_vec())
        }
    }

    pub fn decrease_ttl(&mut self) -> u32 {
        let new_ttl = self.ttl.unwrap() - 1;
        self.ttl = Some(new_ttl);
        new_ttl
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let encoded_message: Vec<u8> = bincode::encode_to_vec(self, bincode::config::standard()).unwrap();
        encoded_message
    }

    pub fn get_from_bytes(bytes: &Vec<u8>) -> Message {
        let (decoded_message, _size): (Message, usize) = bincode::decode_from_slice(bytes, bincode::config::standard()).unwrap();
        decoded_message
    }
}
