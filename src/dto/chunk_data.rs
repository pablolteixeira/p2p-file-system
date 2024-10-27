
use serde::{Deserialize, Serialize};
use bincode;

#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug, Clone)]
pub struct ChunkData {
    pub chunk_id: u8,
    pub data: Vec<u8>,
}
