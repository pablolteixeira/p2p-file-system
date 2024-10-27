use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc::Receiver, Arc, Mutex},
};

use colored::Colorize;

use crate::dto::node::Node;

/// A service that handles TCP communication for a node.
pub struct TcpNodeService {
    pub node: Arc<Mutex<Node>>,
}

impl TcpNodeService {
    /// Creates a new `TcpNodeService` instance.
    pub fn new(node: Arc<Mutex<Node>>) -> Self {
        TcpNodeService { node }
    }

    pub fn listen(&self) {
        // Acquire the node lock to access the IP address
        let mut node_guard = match self.node.lock() {
            Ok(guard) => guard,
            Err(e) => {
                eprintln!("Failed to acquire node lock: {}", e);
                return;
            }
        };

        let mut ip_address = node_guard.ip_address.clone();
        ip_address.set_port(ip_address.port() + 100);

        drop(node_guard);

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
                    println!("Accepted connection from {}", client_addr);

                    let mut buffer = vec![0_u8; 1024];

                    match tcp_stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                // Connection was closed
                                continue;
                            }
                            let received_data = &buffer[..bytes_read];
                            let message = String::from_utf8_lossy(received_data);
                            println!("Received from {}: {}", client_addr, message);

                            // Send response to the client
                            if let Err(e) = tcp_stream.write_all(b"Hi client") {
                                eprintln!("Failed to send response to {}: {}", client_addr, e);
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

    pub fn send(&self, rx: &Receiver<bool>) {
        if rx.recv().is_err() {
            eprintln!("Failed to receive start signal.");
            return;
        }

        let wanted_chunks = {
            let node_guard = match self.node.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    eprintln!("Failed to lock node: {}", e);
                    return;
                }
            };
            node_guard.wanted_chunks.clone()
        };

        println!("{}", format!("{:?}", wanted_chunks).blue());

        for (addr, wanted_chunk) in &wanted_chunks {
            let _ = wanted_chunk;

            let mut socket_tcp = match TcpStream::connect(addr) {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to connect to {}: {}", addr, e);
                    continue;
                }
            };

            if let Err(e) = socket_tcp.write_all(b"Hi server!") {
                eprintln!("Failed to send request to {}: {}", addr, e);
                continue;
            }

            let mut buffer = vec![0_u8; 1024];
            match socket_tcp.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        eprintln!("Connection closed by {}", addr);
                        continue;
                    }
                    println!(
                        "Received from {}: {}",
                        addr,
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );
                }
                Err(e) => {
                    eprintln!("Failed to read response from {}: {}", addr, e);
                }
            }
        }
    }
}
