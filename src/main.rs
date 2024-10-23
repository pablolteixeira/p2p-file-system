pub mod dto;
pub mod server_service;

use clap::Parser;
use dto::message::{Message};
use server_service::file_utils::FileUtils;
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

    let mut file_utils = FileUtils::new(args.node_id);
    file_utils.startup();

    let udp_node_service: Arc<UdpNodeService> = Arc::new(UdpNodeService::new(Arc::clone(&node)));

    let udp_node_service_clone = Arc::clone(&udp_node_service);
    let udp_node_service_thread = thread::spawn(move || {
        let _ = udp_node_service_clone.listen();
    });

    let cli_thread = {
        let udp_node_service_clone = Arc::clone(&udp_node_service);
        let node_clone = Arc::clone(&node);
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
                println!("\nYou entered: {}", command); 

                let sender_ip = {
                    let node_guard = node_clone.lock().expect("Failed to lock node");
                    node_guard.ip_address
                };

                let new_message = Message::new(String::from("test.txt"), sender_ip.to_string());

                let _ = udp_node_service_clone.send(&new_message.get_bytes());
            }
        })
    };

    udp_node_service_thread.join().unwrap();
    cli_thread.join().unwrap();

    Ok(())
}
