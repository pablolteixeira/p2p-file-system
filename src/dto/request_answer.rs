use serde::{Serialize, Deserialize};
use bincode::{self, Encode};

#[derive(Serialize, Deserialize, Encode, bincode::Decode, Debug)]
pub struct RequestAnswer {
    pub filename: String,
    pub file_owner_ip: String,
    pub number_of_chunks_present: u64,
    pub chunks_present: Vec<u64>,
    pub transfer_speed: u32,
}

impl RequestAnswer {
    pub fn new(
        filename: String,
        file_owner_ip: String,
        number_of_chunks_present: u64,
        chunks_present: Vec<u64>,
        transfer_speed: u32
    ) -> Self {
        RequestAnswer {
            filename,
            file_owner_ip,
            number_of_chunks_present,
            chunks_present,
            transfer_speed,
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, bincode::config::standard())
            .expect("Failed to serialize RequestAnswer to bytes.")
    }

    pub fn get_from_bytes(bytes: &[u8]) -> Self {
        let (decoded_message, _size): (RequestAnswer, usize) =
            bincode::decode_from_slice(bytes, bincode::config::standard())
                .expect("Failed to deserialize bytes into RequestAnswer.");

        decoded_message
    }
}
