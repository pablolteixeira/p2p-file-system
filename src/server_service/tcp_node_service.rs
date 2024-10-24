use std::{net::TcpStream, sync::{Arc, Mutex}};

use crate::dto::node::Node;

struct TcpNodeService {
    pub node: Arc<Mutex<Node>>,
    pub socket_tcp: TcpStream
}

impl TcpNodeService {
    fn new(node: Arc<Mutex<Node>>) -> Self {
        

        TcpNodeService {
            node,
            
        }
    }
}