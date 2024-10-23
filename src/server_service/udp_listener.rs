use std::net::UdpSocket;
use crate::dto::message::Message;


use crate::dto::node::Node;

pub struct UdpListener {
    pub node: Node,

}

impl UdpListener {
    pub fn new(node: Node) -> Self {
        let node: Node = node;
        UdpListener { node }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        let socket: UdpSocket = UdpSocket::bind(&self.node.ip_address)?;

        println!("Node {} está escutando...", self.node.node_id);

        let mut buffer = vec![0_u8; 1024];


        loop {
            let (n, client_addr) = socket.recv_from(&mut buffer)?;
            let mut received_message: Message = Message::get_from_bytes(&buffer);

            let message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);

            println!(
                "Received message from {:?}: {}",
                client_addr,
                message.to_string()
            );

            let serialized_message = received_message.get_bytes();
            let byte_slice: &[u8] = &serialized_message[..];
            socket.send_to(b"buf", &client_addr)?;

            let mut current_number_of_jumps:u32 = socket.ttl()?;

            if current_number_of_jumps > 1{
                current_number_of_jumps -= 1;
                socket.set_ttl(current_number_of_jumps)?;
                for (key, value) in self.node.neighbors_hashmap.iter() {
                    print!("Flooding messages to {}: ", key);
                    socket.send_to(byte_slice, value)?;}
            }
            else{
                print!("Finished TTL {}", message);
            }



        }
    }
}
