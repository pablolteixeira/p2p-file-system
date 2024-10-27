use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

use crate::{
    dto::descritor::Descritor,
    server_service::file_utils::FileUtils,
};

/// Represents a node in the network.
pub struct Node {
    pub node_id: u128,
    pub ip_address: SocketAddr,
    pub ip_address_tcp: SocketAddr,
    pub transfer_speed: u32,
    pub descritor: Descritor,
    pub neighbors_hashmap: HashMap<u128, SocketAddr>,
    pub file_utils: FileUtils,
    pub wanted_chunks: HashMap<String, (u32, Vec<u8>)>, // Key: Addr, Value: (transfer_speed, chunks)
    pub chunks_counter: HashMap<String, HashSet<u8>>,   // Key: Message ID, Value: Set of chunks
    pub file_name: Option<String>,
    pub total_chunks: u8,
}

impl Node {
    /// Creates a new `Node` instance.
    pub fn new(node_id: u128) -> Self {
        let descritor = Descritor::new(node_id);
        let (ip_address, transfer_speed, neighbors_hashmap) = descritor.parse_file();
        let file_utils = FileUtils::new(node_id);

        let ip_address_udp = ip_address.ip();
        let udp_port = ip_address.port();
        let ip_address_tcp = SocketAddr::new(ip_address_udp, udp_port + 100);

        let wanted_chunks = HashMap::<String, (u32, Vec<u8>)>::new();
        let chunks_counter = HashMap::<String, HashSet<u8>>::new();

        Node {
            node_id,
            ip_address,
            ip_address_tcp,
            transfer_speed,
            descritor,
            neighbors_hashmap,
            file_utils,
            wanted_chunks,
            chunks_counter,
            file_name: None,
            total_chunks: 0,
        }
    }
}
