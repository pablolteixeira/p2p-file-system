use crate::dto::descritor::Descritor;

use std::collections::HashMap;
use crate::server_service::file_utils::FileUtils;
use std::net::SocketAddr;

pub struct Node {
    pub node_id: u128,
    pub ip_address: SocketAddr,
    pub transfer_speed: u32,
    pub descritor: Descritor,
    pub neighbors_hashmap: HashMap<u128, SocketAddr>,
    pub file_utils: FileUtils,
}

impl Node {
    pub fn new(node_id: u128) -> Self {
        let descritor: Descritor = Descritor::new(node_id);
        let (ip_address, transfer_speed, neighbors_hashmap) = descritor.parse_file();
        let mut file_utils = FileUtils::new(node_id);
        file_utils.startup();
        
        Node {
            node_id,
            ip_address,
            transfer_speed,
            descritor,
            neighbors_hashmap,
            file_utils,
        }
    }
}