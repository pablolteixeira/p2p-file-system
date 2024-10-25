use std::{net::UdpSocket, time::Duration};
use crate::dto::{message::{Message, MessageType}, node::Node};

use std::sync::{Arc, Mutex};

use colored::Colorize;

pub struct UdpNodeService {
    pub node: Arc<Mutex<Node>>,
    pub socket_udp: UdpSocket,
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
            socket_udp,
        }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        println!("Node {} - UDP está escutando...", self.node.lock().unwrap().ip_address);

        let mut buffer = vec![0_u8; 1024];

        loop {
            match self.socket_udp.recv_from(&mut buffer) {
                Ok((n, sender)) => {
                    let _ = String::from_utf8_lossy(&buffer[..n]);
                    let mut received_message: Message = Message::get_from_bytes(&buffer);

                    if received_message.message_type == MessageType::Flooding {
                        let current_ttl = received_message.decrease_ttl();

                        println!("Received message from {} - ttl -> {}", sender.to_string(), received_message.ttl.unwrap());
                    
                        let node = self.node.lock().unwrap();

                        if current_ttl > 0{
                            let result = node.file_utils.get_chunks_from_folder(&received_message.filename.clone().unwrap());

                            if result.len() > 0 {
                                std::thread::sleep(Duration::from_millis(1000));
                                println!("Node {} has the following chunks -> {:?}", node.node_id, result);
                                let chunks_found_message = Message::new_chunks_found(
                                    received_message.id.clone().unwrap(),
                                    node.ip_address_tcp.clone(),
                                    received_message.chunk_amount,
                                    &result,
                                    node.transfer_speed
                                );
    
                                let serialized_message = chunks_found_message.get_bytes();
                                let byte_slice: &[u8] = &serialized_message[..];
    
                                let format_string = format!("Sending chunks to {} - Chunks -> {:?}", received_message.sender_ip.clone().unwrap(), result).red(); 
                                println!("{}", format_string);
                                std::thread::sleep(Duration::from_millis(1000));
                                self.socket_udp.send_to(byte_slice, received_message.sender_ip.clone().unwrap())?;
                            }
    
                            let serialized_message = received_message.get_bytes();
                            let byte_slice: &[u8] = &serialized_message[..];

                            for (key, value) in node.neighbors_hashmap.iter() {
                                println!("Flooding messages to {} - {}", key, value.to_string());
                                self.socket_udp.send_to(byte_slice, value)?;}
                        } else {
                            println!("Finished TTL {:?}", received_message);
                        }
                    } else if received_message.message_type == MessageType::ChunksFound {
                        let _ = String::from_utf8_lossy(&buffer[..n]);
                        let received_message: Message = Message::get_from_bytes(&buffer);
                        
                        let format_string = format!("Received chunks from {}:{} - Chunks -> {:?} - Ip TCP {}", sender.ip(), sender.port(), received_message.chunks.clone().unwrap(), received_message.sender_ip_tcp.unwrap()).green(); 

                        let mut node = self.node.lock().unwrap();

                        let key = format!("{}:{}", received_message.sender_ip_tcp.unwrap().ip(), received_message.sender_ip_tcp.unwrap().port());

                        node.wanted_chunks.insert(key, (received_message.transfer_speed.unwrap(), received_message.chunks.clone().unwrap()));
                        
                        if let Some(chunks_hash_set) = node.chunks_counter.get_mut(&received_message.id.clone().unwrap()) {
                            for chunk in received_message.chunks.unwrap().iter() {
                                chunks_hash_set.insert(chunk.clone());
                            }
                        }                        
                        
                        if node.chunks_counter.get(&received_message.id.unwrap()).unwrap().len() == received_message.chunk_amount.into() {
                            
                        }

                        println!("{:?}", format_string);
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
        println!("Sending message");

        let node = self.node.lock().unwrap();
        for (key, value) in node.neighbors_hashmap.iter() {
            let _n = self.socket_udp.send_to(message, value)?;
            
            println!("Flooding messages to {} - {}", key, value.to_string());
        }
        
        Ok(())
    }
}