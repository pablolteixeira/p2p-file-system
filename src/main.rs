pub mod dto;
pub mod server_service;

use clap::Parser;
use dto::node::Node;
use server_service::UdpNodeService;

use std::thread;
use std::io::{self, Write};

use std::sync::{Arc, Mutex};

#[derive(Parser)]
struct Cli {
    node_id: u128,
}

fn main() -> std::io::Result<()> {
    let args: Cli = Cli::parse();
    let node: Arc<Mutex<Node>> = Arc::new(Mutex::new(Node::new(args.node_id)));

    let udp_listener: Arc<UdpNodeService> = Arc::new(UdpNodeService::new(node));

    let udp_listener_clone = Arc::clone(&udp_listener);
    let udp_listener_thread = thread::spawn(move || {
        let _ = udp_listener_clone.listen();
    });

    let cli_thread = {
        let udp_listener = Arc::clone(&udp_listener);
        thread::spawn(move || {
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

                let _ = udp_listener.send(input.as_bytes());
            }
        })
    };

    udp_listener_thread.join().unwrap();
    cli_thread.join().unwrap();

    Ok(())
}
