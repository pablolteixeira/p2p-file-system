use std::{io::Read, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};

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
                },
                Err(e) => {

                }   
            }
        }
    }

    pub fn send(&self) {
        let node = self.node.lock().unwrap();
        
        println!("{:?}", node.wanted_chunks);
    }
}