use crate::dto::descritor::Descritor;

use std::collections::HashMap;
use std::{io::{Read, Write}, net::SocketAddr};
use socket2::{Socket, Domain, Type, Protocol};

pub struct Node {
    node_id: u128,
    ip_address: SocketAddr,
    transfer_speed: u32,
    descritor: Descritor,
    neighbors_hashmap: HashMap<u128, SocketAddr>
}

impl Node {
    pub fn new(node_id: u128) -> Self {
        let descritor = Descritor::new(node_id);
        let (ip_address, transfer_speed, neighbors_hashmap) = descritor.parse_file();

        /*neighbors_hashmap.keys().for_each(|f| {
            let value = neighbors_hashmap.get(f).unwrap();
            println!("{} - {}", f, value);
        });

        println!("Ip Address -> {ip_address}\nTransfer Speed -> {transfer_speed}");*/

        Node {
            node_id,
            ip_address,
            transfer_speed,
            descritor,
            neighbors_hashmap
        }
    }
}