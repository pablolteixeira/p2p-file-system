use rand::{distributions::Alphanumeric, Rng};
use serde::{Serialize, Deserialize};
use bincode::{self, Encode};

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct Message {
    pub id: String,
    pub ttl: u32,
    pub filename: String,
    pub sender_ip: String,
}

impl Message {
    pub fn new(filename: String, sender_ip: String) -> Message {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let ttl: u32 = 5;

        Message { id, ttl, filename, sender_ip }
    }

    pub fn decrease_ttl(&mut self) -> u32 {
        self.ttl -= 1;
        self.ttl
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
