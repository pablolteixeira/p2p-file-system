use rand::{distributions::Alphanumeric, Rng};
use serde::{Serialize, Deserialize};
use bincode::{self, Encode};

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug)]
pub struct Message {
    pub id: String,
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

        Message { id, filename, sender_ip }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn get_sender_ip(&self) -> &String {
        &self.sender_ip
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn set_sender_ip(&mut self, sender_ip: String) {
        self.sender_ip = sender_ip;
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
