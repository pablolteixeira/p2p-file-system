use std::{f32::consts::E, net::UdpSocket, time::Duration};
use crate::dto::node::Node;

use std::sync::{Arc, Mutex};

pub struct UdpNodeService {
    pub node: Arc<Mutex<Node>>,
    pub socket_udp: UdpSocket
}

impl UdpNodeService {
    pub fn new(node: Arc<Mutex<Node>>) -> Self {
        let ip_address = node.lock().unwrap().ip_address.clone();
        let socket_udp = UdpSocket::bind(ip_address).expect("Failed to bind socket");

        socket_udp
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");

        UdpNodeService { 
            node,
            socket_udp
        }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        println!("Node {} está escutando...", self.node.lock().unwrap().ip_address);

        let mut buffer = vec![0_u8; 1024];

        loop {
            match self.socket_udp.recv(&mut buffer) {
                Ok(n) => {
                    let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);
                    
                    println!("Received message {}", message.to_string());

                    let node = self.node.lock().unwrap();

                    for (key, value) in node.neighbors_hashmap.iter() {
                        print!("Flooding messages to {}: ", key);
                        self.socket_udp.send_to(message.as_bytes(), value)?;
                    }
                },
                Err(ref e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        // Espera um pouco antes de tentar receber novamente (evita busy-waiting)
                        std::thread::sleep(Duration::from_millis(10));
                    } else {
                        println!("Erro ao receber mensagem: {}", e);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn send(&self, message: &[u8]) -> std::io::Result<()> {
        println!("Enviando mensagem");

        let node = self.node.lock().unwrap();
        for (_, value) in node.neighbors_hashmap.iter() {
            self.socket_udp.connect(value)?;
            let n = self.socket_udp.send(message)?;
            
            println!("Written buffer: {:?}", String::from_utf8_lossy(&message[0..n]));
        }
        
        Ok(())
    }
}