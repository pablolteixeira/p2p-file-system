pub mod dto;
use std::{mem::MaybeUninit, net::UdpSocket};

use clap::Parser;
use dto::node::Node;
mod server_service;
use server_service::UdpListener;

use socket2::{Domain, Protocol, Socket, Type};

#[derive(Parser)]
struct Cli {
    node_id: u128,
}

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();

    let node: Node = Node::new(args.node_id);

    let udp_listener: UdpListener = UdpListener::new(node);

    udp_listener.listen()
}
