use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc::Receiver, Arc, Mutex},
    time::Duration,
    collections::HashSet,
};

use colored::Colorize;

use crate::dto::{message::{Message, MessageType}, node::Node};

/// A service that handles TCP communication for a node.
pub struct TcpNodeService {
    pub node: Arc<Mutex<Node>>,
}

impl TcpNodeService {
    /// Creates a new `TcpNodeService` instance.
    pub fn new(node: Arc<Mutex<Node>>) -> Self {
        TcpNodeService { node }
    }

    /// Starts listening for incoming TCP connections.
    pub fn listen(&self) {
        // Acquire the node lock to access the IP address
        let ip_address = {
            let node_guard = match self.node.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    eprintln!("Failed to acquire node lock: {}", e);
                    return;
                }
            };
            node_guard.ip_address_tcp.clone()
        };

        // Bind the listener to the IP address
        let listener = match TcpListener::bind(ip_address) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind TCP listener: {}", e);
                return;
            }
        };

        println!(
            "Node {}:{} - TCP listening...",
            listener.local_addr().unwrap().ip(),
            listener.local_addr().unwrap().port()
        );

        loop {
            match listener.accept() {
                Ok((mut tcp_stream, client_addr)) => {
                    // Read the incoming message
                    let mut buffer = vec![0_u8; 1024];
                    match tcp_stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                continue;
                            }
                            let received_message = Message::get_from_bytes(&buffer[..bytes_read]);

                            match received_message.message_type {
                                MessageType::ChunkRequest => {
                                    // Handle the chunk request
                                    let file_name = received_message.filename.unwrap();
                                    let requested_chunks = received_message.chunks.unwrap();

                                    println!(
                                        "Received chunk request from {} for chunks {:?} of file {}",
                                        client_addr, requested_chunks, file_name
                                    );

                                    // Retrieve the requested chunks from the file system
                                    let node = self.node.lock().unwrap();
                                    let chunks_data = node.file_utils.get_chunks_data(&file_name, &requested_chunks);

                                    // Send the chunks back to the requester
                                    if let Err(e) = tcp_stream.write_all(&chunks_data) {
                                        eprintln!("Failed to send chunks to {}: {}", client_addr, e);
                                    } else {
                                        println!("Sent chunks {:?} to {}", requested_chunks, client_addr);
                                    }
                                }
                                _ => {
                                    // Handle other message types if necessary
                                    eprintln!("Received unexpected message type from {}", client_addr);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from {}: {}", client_addr, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Sends data to other nodes as specified in `wanted_chunks`.
    pub fn send(&self, rx: &Receiver<bool>) {
        // Wait for the signal indicating all chunk information has been received
        if rx.recv().is_err() {
            eprintln!("Failed to receive start signal.");
            return;
        }

        // Wait for an additional 5 seconds
        std::thread::sleep(Duration::from_secs(5));

        // Lock the node to access shared data after receiving the signal
        let (wanted_chunks, file_name, total_chunks) = {
            let node_guard = match self.node.lock() {
                Ok(node) => node,
                Err(e) => {
                    eprintln!("Failed to lock node: {}", e);
                    return;
                }
            };
            (
                node_guard.wanted_chunks.clone(),
                node_guard.file_name.clone().unwrap(),
                node_guard.total_chunks, // Assuming you store the total number of chunks
            )
        };

        // Log the wanted chunks
        println!("{}", format!("Wanted chunks: {:?}", wanted_chunks).blue());

        // Sort peers by transfer speed in descending order
        let mut peers: Vec<_> = wanted_chunks.iter().collect();
        peers.sort_by_key(|(_, (speed, _))| std::cmp::Reverse(*speed));

        // Keep track of downloaded chunks to avoid duplicates
        let mut downloaded_chunks = HashSet::new();

        // Iterate over peers and request chunks
        for (addr, (_speed, chunks)) in peers {
            // Filter out chunks that have already been downloaded
            let chunks_to_request: Vec<u8> = chunks
                .iter()
                .filter(|chunk| !downloaded_chunks.contains(*chunk))
                .cloned()
                .collect();

            if chunks_to_request.is_empty() {
                continue;
            }

            println!("Connecting to {} to request chunks {:?}", addr, chunks_to_request);

            // Connect to the node via TCP
            let mut socket_tcp = match TcpStream::connect(addr) {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to connect to {}: {}", addr, e);
                    continue;
                }
            };

            // Create a message requesting the chunks
            let request_message = Message::new_chunk_request(
                file_name.clone(),
                chunks_to_request.clone(),
            );

            // Send the request message
            if let Err(e) = socket_tcp.write_all(&request_message.get_bytes()) {
                eprintln!("Failed to send chunk request to {}: {}", addr, e);
                continue;
            }

            // Read the response and save the chunks
            let mut buffer = vec![0_u8; 1024 * 1024]; // Adjust buffer size as needed
            match socket_tcp.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        eprintln!("Received no data from {}", addr);
                        continue;
                    }
                    // Process the received data
                    let chunk_data = &buffer[..bytes_read];

                    // Save the chunk data
                    let node = self.node.lock().unwrap();
                    node.file_utils.save_chunks(&file_name, chunk_data);

                    // Update the downloaded_chunks set
                    downloaded_chunks.extend(chunks_to_request.clone());

                    println!("Downloaded chunks {:?} from {}", chunks_to_request, addr);
                }
                Err(e) => {
                    eprintln!("Failed to read chunks from {}: {}", addr, e);
                }
            }

            // Check if all chunks have been downloaded
            if downloaded_chunks.len() == total_chunks as usize {
                println!("All chunks have been downloaded.");
                break;
            }
        }

        if downloaded_chunks.len() < total_chunks as usize {
            println!("Could not download all chunks.");
        }
    }
}
