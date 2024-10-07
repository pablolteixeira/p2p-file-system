use crate::dto::descritor::Descritor;

use socket2::{Domain, Protocol, Socket, Type};
use std::collections::HashMap;
use std::{
    io::{Read, Write},
    net::SocketAddr,
};

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

        /*
        neighbors_hashmap.keys().for_each(|f| {
            let value = neighbors_hashmap.get(f).unwrap();
            println!("{} - {}", f, value);
        });

        println!("Ip Address -> {ip_address}\nTransfer Speed -> {transfer_speed}");
        */
        
        Node {
            node_id,
            ip_address,
            transfer_speed,
            descritor,
            neighbors_hashmap,
        }
    }
}
