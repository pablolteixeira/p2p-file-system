pub mod dto;
pub mod server_service;

use clap::Parser;
use dto::node::Node;
use server_service::UdpListener;

use std::thread;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    node_id: u128,
}

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();
    let node: Node = Node::new(args.node_id);

    let udp_listener: UdpListener = UdpListener::new(node);

    let udp_listener_thread = thread::spawn(move || {
        let _ = udp_listener.listen();
    });

    let cli_thread = thread::spawn(move || {
        loop {
            print!("Enter command: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            
            let command = input.trim();
            if command == "quit" {
                break;
            }
            println!("You entered: {}", command);
        }
    });

    udp_listener_thread.join().unwrap();
    cli_thread.join().unwrap();

    Ok(())
}
