use crate::dto::descritor::Descritor;

use std::collections::HashMap;
use std::net::SocketAddr;

pub struct Node {
    pub node_id: u128,
    pub ip_address: SocketAddr,
    pub transfer_speed: u32,
    pub descritor: Descritor,
    pub neighbors_hashmap: HashMap<u128, SocketAddr>,
}

impl Node {
    pub fn new(node_id: u128) -> Self {
        let descritor: Descritor = Descritor::new(node_id);
        let (ip_address, transfer_speed, neighbors_hashmap) = descritor.parse_file();
        
        Node {
            node_id,
            ip_address,
            transfer_speed,
            descritor,
            neighbors_hashmap,
        }
    }
}
