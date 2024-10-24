use std::{net::UdpSocket, time::Duration};
use crate::dto::{node::Node, message::Message};

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
            match self.socket_udp.recv_from(&mut buffer) {
                Ok((n, sender)) => {
                    let _ = String::from_utf8_lossy(&buffer[..n]);
                    let mut received_message: Message = Message::get_from_bytes(&buffer);

                    let current_ttl = received_message.decrease_ttl();

                    println!("Received message from {} - ttl -> {}", sender.to_string(), received_message.ttl);
                
                    let node = self.node.lock().unwrap();
                    
                    let serialized_message = received_message.get_bytes();
                    let byte_slice: &[u8] = &serialized_message[..];

                    if current_ttl > 0{
                        for (key, value) in node.neighbors_hashmap.iter() {
                            println!("Flooding messages to {} - {}", key, value.to_string());
                            self.socket_udp.send_to(byte_slice, value)?;}
                    }
                    else{
                        println!("Finished TTL {:?}", received_message);
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
        for (key, value) in node.neighbors_hashmap.iter() {
            self.socket_udp.connect(value)?;
            let _n = self.socket_udp.send(message)?;
            
            println!("Flooding messages to {} - {}", key, value.to_string());

            //println!("Written buffer: {:?}", String::from_utf8_lossy(&message[0..n]));
        }
        
        Ok(())
    }
}