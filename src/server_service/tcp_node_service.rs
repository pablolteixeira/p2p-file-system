use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{mpsc::Receiver, Arc, Mutex}};

use colored::Colorize;

use crate::dto::node::Node;

pub struct TcpNodeService {
    pub node: Arc<Mutex<Node>>,
}

impl TcpNodeService {
    pub fn new(node: Arc<Mutex<Node>>) -> Self {
        TcpNodeService {
            node,
        }
    }

    pub fn listen(&self) {
        let mut ip_address = self.node.lock().unwrap().ip_address.clone();
        let new_port = ip_address.port() + 100;

        ip_address.set_port(new_port);

        let listener = TcpListener::bind(ip_address).unwrap();

        println!("Node {}:{} - TCP está escutando...", listener.local_addr().unwrap().ip(), listener.local_addr().unwrap().port());

        let mut buffer = vec![0_u8; 1024];

        loop {
            match listener.accept() {
                Ok((mut tcp_stream, client)) => {
                    let _ = tcp_stream.read_to_end(&mut buffer);
                    let _ = String::from_utf8_lossy(&buffer[..]);
                    println!("{:?}", buffer);

                    let _ = tcp_stream.write(b"Hi client");
                },
                Err(e) => {

                }   
            }
        }
    }

    pub fn send(&self, rx: &Receiver<bool>) {
        if rx.recv().is_err() {
            eprintln!("Failed to receive start signal.");
            return;
        }

        // Lock the node to access shared data after receiving the signal
        let wanted_chunks = {
            // Acquire the lock and extract needed information
            let node_guard = match self.node.lock() {
                Ok(node) => node,
                Err(e) => {
                    eprintln!("Failed to lock node: {}", e);
                    return;
                }
            };
            node_guard.wanted_chunks.clone() // Clone to release the lock sooner
        };

        println!("{}", format!("{:?}", wanted_chunks).blue());

        for (addr, wanted_chunk) in &wanted_chunks {
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
                    // Output the received message
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