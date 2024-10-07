use std::{mem::MaybeUninit, net::UdpSocket};

use socket2::{Domain, Protocol, Socket, Type};

use crate::dto::node::Node;

pub struct UdpListener {
    pub node: Node,
}

impl UdpListener {
    pub fn new(node: Node) -> Self {
        let node: Node = node;

        UdpListener { node }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        let socket: UdpSocket = UdpSocket::bind(&self.node.ip_address)?;

        println!("Node {} está escutando...", self.node.node_id);

        let mut buffer = vec![0_u8; 1024];

        loop {
            let (n, client_addr) = socket.recv_from(&mut buffer)?;

            let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);

            println!(
                "Received message from {:?}: {}",
                client_addr,
                message.to_string()
            );

            socket.send_to(b"buf", &client_addr)?;

            for (key, value) in self.node.neighbors_hashmap.iter() {
                print!("Flooding messages to {}: ", key);
                socket.send_to(message.as_bytes(), value)?;
            }
        }
    }
}
