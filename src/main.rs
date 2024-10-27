pub mod dto;
pub mod server_service;

use clap::Parser;
use dto::message::Message;
use dto::metadata_parser::MetadataParser;
use dto::node::Node;
use server_service::TcpNodeService;
use server_service::UdpNodeService;

use std::io::{self, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Command-line arguments structure.
#[derive(Parser)]
struct Cli {
    node_id: u128,
}

fn main() -> io::Result<()> {
    let args: Cli = Cli::parse();

    // Initialize the shared Node instance.
    let node = Arc::new(Mutex::new(Node::new(args.node_id)));

    // Create a communication channel for inter-thread messaging.
    let (tx, rx) = mpsc::channel::<bool>();

    // Start the UDP node service in a separate thread.
    let udp_node_service = Arc::new(UdpNodeService::new(Arc::clone(&node)));
    let udp_thread = start_udp_service(Arc::clone(&udp_node_service), tx);

    // Start the TCP node service in a separate thread for listening.
    let tcp_node_service = Arc::new(TcpNodeService::new(Arc::clone(&node)));
    let tcp_listener_service = Arc::clone(&tcp_node_service);
    let tcp_listener_thread = thread::spawn(move || {
        tcp_listener_service.listen();
    });

    // Start the TCP sender service in a separate thread for sending requests.
    let tcp_sender_service = Arc::clone(&tcp_node_service);
    let tcp_sender_thread = thread::spawn(move || {
        tcp_sender_service.send(&rx);
    });

    // Start the CLI thread for user interaction.
    let cli_thread = start_cli_thread(
        Arc::clone(&udp_node_service),
        Arc::clone(&tcp_node_service),
        Arc::clone(&node),
    );

    // Wait for all threads to complete.
    udp_thread.join().expect("UDP service thread panicked");
    tcp_listener_thread.join().expect("TCP listener thread panicked");
    tcp_sender_thread.join().expect("TCP sender thread panicked");
    cli_thread.join().expect("CLI thread panicked");

    Ok(())
}

/// Starts the UDP service thread.
fn start_udp_service(
    udp_service: Arc<UdpNodeService>,
    tx: mpsc::Sender<bool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if let Err(e) = udp_service.listen(tx) {
            eprintln!("UDP service encountered an error: {}", e);
        }
    })
}

/// Starts the CLI thread for user input.
fn start_cli_thread(
    udp_service: Arc<UdpNodeService>,
    tcp_service: Arc<TcpNodeService>,
    node: Arc<Mutex<Node>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        if let Err(e) = process_user_input(
            Arc::clone(&udp_service),
            Arc::clone(&tcp_service),
            Arc::clone(&node),
        ) {
            eprintln!("Error processing user input: {}", e);
        }
    })
}

/// Processes user input from the CLI.
fn process_user_input(
    udp_service: Arc<UdpNodeService>,
    _tcp_service: Arc<TcpNodeService>,
    node: Arc<Mutex<Node>>,
) -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter the .p2p file name (or 'quit' to exit): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let command = input.trim();
    if command.eq_ignore_ascii_case("quit") {
        println!("Exiting...");
        std::process::exit(0);
    } else if !command.is_empty() {
        // Retrieve the sender's IP address.
        let sender_ip = {
            let node = node.lock().map_err(|e| format!("Failed to lock node: {}", e))?;
            node.ip_address.clone()
        };

        // Parse the metadata file.
        let metadata_parser = MetadataParser::new(command);
        let (file_name, chunks, ttl) = metadata_parser.parse();

        // Create a new message for flooding.
        let new_message = Message::new_flooding(file_name, sender_ip, chunks, ttl);

        // Send the message via UDP service.
        udp_service.send(&new_message.get_bytes())?;
    }
    Ok(())
}
