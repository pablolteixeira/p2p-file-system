use std::{
    collections::{HashSet, VecDeque},
    net::UdpSocket,
    sync::{Arc, Mutex, mpsc::Sender},
    time::Duration,
};

use colored::Colorize;

use crate::dto::{
    message::{Message, MessageType},
    node::Node,
};

pub struct UdpNodeService {
    pub node: Arc<Mutex<Node>>,
    pub socket_udp: UdpSocket,
    pub cache: Arc<Mutex<VecDeque<String>>>,
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
            cache: Arc::new(Mutex::new(VecDeque::with_capacity(10))),
        }
    }

    pub fn listen(&self, tx: Sender<bool>) -> std::io::Result<()> {
        println!(
            "Node {} - UDP listening...",
            self.node.lock().unwrap().ip_address
        );

        let mut buffer = vec![0_u8; 1024];

        loop {
            match self.socket_udp.recv_from(&mut buffer) {
                Ok((n, sender)) => {
                    let mut received_message: Message = Message::get_from_bytes(&buffer[..n]);

                    if received_message.message_type == MessageType::Flooding {
                        let message_id = received_message.id.clone().unwrap();
                        let mut cache = self.cache.lock().unwrap();

                        if cache.contains(&message_id) {
                            println!(
                                "Message with ID {} is already processed, skipping propagation",
                                message_id
                            );
                            continue; // Skip this message, already processed
                        }

                        if cache.len() >= 10 {
                            cache.pop_front();
                        }

                        cache.push_back(message_id.clone());

                        let current_ttl = received_message.decrease_ttl();
                        println!(
                            "Received flooding message from {} - ttl -> {}",
                            sender.to_string(),
                            current_ttl
                        );

                        let node = self.node.lock().unwrap();

                        if current_ttl > 0 {
                            let result = node
                                .file_utils
                                .get_chunks_from_folder(&received_message.filename.clone().unwrap());

                            if !result.is_empty() {
                                std::thread::sleep(Duration::from_millis(1000));
                                println!(
                                    "Node {} has the following chunks -> {:?}",
                                    node.node_id, result
                                );
                                let chunks_found_message = Message::new_chunks_found(
                                    received_message.id.clone().unwrap(),
                                    received_message.filename.clone().unwrap(),
                                    node.ip_address_tcp.clone(),
                                    received_message.chunk_amount,
                                    &result,
                                    node.transfer_speed,
                                );

                                let serialized_message = chunks_found_message.get_bytes();

                                let format_string = format!(
                                    "Sending chunks to {} - Chunks -> {:?}",
                                    received_message.sender_ip.clone().unwrap(),
                                    result
                                )
                                    .red();
                                println!("{}", format_string);
                                std::thread::sleep(Duration::from_millis(1000));
                                self.socket_udp.send_to(
                                    &serialized_message,
                                    received_message.sender_ip.clone().unwrap(),
                                )?;
                            }

                            let serialized_message = received_message.get_bytes();

                            for (key, value) in node.neighbors_hashmap.iter() {
                                println!("Flooding messages to {} - {}", key, value.to_string());
                                self.socket_udp.send_to(&serialized_message, value)?;
                            }
                        } else {
                            println!("Finished TTL {:?}", received_message);
                        }
                    } else if received_message.message_type == MessageType::ChunksFound {
                        let received_message: Message = Message::get_from_bytes(&buffer[..n]);

                        let format_string = format!(
                            "Received chunks from {}:{} - Chunks -> {:?} - Ip TCP {}",
                            sender.ip(),
                            sender.port(),
                            received_message.chunks.clone().unwrap(),
                            received_message.sender_ip_tcp.unwrap()
                        )
                            .green();

                        let mut node = self.node.lock().unwrap();

                        let key = format!(
                            "{}:{}",
                            received_message.sender_ip_tcp.unwrap().ip(),
                            received_message.sender_ip_tcp.unwrap().port()
                        );

                        node.wanted_chunks.insert(
                            key,
                            (
                                received_message.transfer_speed.unwrap(),
                                received_message.chunks.clone().unwrap(),
                            ),
                        );

                        if let Some(chunks_hash_set) =
                            node.chunks_counter.get_mut(&received_message.id.clone().unwrap())
                        {
                            for chunk in received_message.chunks.clone().unwrap().iter() {
                                chunks_hash_set.insert(*chunk);
                            }
                        } else {
                            let mut hash_set = HashSet::<u8>::new();

                            for chunk in received_message.chunks.clone().unwrap().iter() {
                                hash_set.insert(*chunk);
                            }

                            node.chunks_counter
                                .insert(received_message.id.clone().unwrap(), hash_set);
                        }

                        println!(
                            "Chunks received: {} / {}",
                            node.chunks_counter
                                .get(&received_message.id.clone().unwrap())
                                .unwrap()
                                .len(),
                            received_message.chunk_amount
                        );

                        if node
                            .chunks_counter
                            .get(&received_message.id.clone().unwrap())
                            .unwrap()
                            .len()
                            == received_message.chunk_amount as usize
                        {
                            node.file_name = Some(received_message.filename.clone().unwrap());
                            node.total_chunks = received_message.chunk_amount;
                            let _ = tx.send(true);
                        }

                        println!("{}", format_string);
                    }
                }
                Err(ref e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        // Wait a bit before trying again to avoid busy-waiting
                        std::thread::sleep(Duration::from_millis(10));
                    } else {
                        println!("Error receiving message: {}", e);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn send(&self, message: &[u8]) -> std::io::Result<()> {
        println!("Sending message via UDP");

        let node = self.node.lock().unwrap();
        for (key, value) in node.neighbors_hashmap.iter() {
            let _n = self.socket_udp.send_to(message, value)?;

            println!("Flooding messages to {} - {}", key, value.to_string());
        }

        Ok(())
    }
}
