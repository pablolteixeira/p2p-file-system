pub mod dto;
use std::{mem::MaybeUninit, net::UdpSocket};

use dto::node::Node;
use clap::Parser;

use socket2::{Socket, Domain, Type, Protocol};

#[derive(Parser)]
struct Cli {
    node_id: u128
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let node = Node::new(args.node_id);

    let socket = UdpSocket::bind(&node.ip_address)?;

    println!("Node {} está escutando...", node.node_id);

    let mut buffer = vec![0_u8; 1024];

    loop {
        let (n, client_addr) = socket.recv_from(&mut buffer)?;

        println!("Received message from {:?}: {}", client_addr, String::from_utf8_lossy(&buffer[..n]));

        socket.send_to(b"buf", &client_addr)?;
    }
}
