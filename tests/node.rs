use p2p_file_system::dto::node::Node;
use std::collections::HashMap;
use std::net::SocketAddr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_with_id_0() {
        let node = Node::new(0);

        let expected_ip_address: SocketAddr = "127.0.0.1:6000".parse().unwrap();
        let expected_transfer_speed: u32 = 100;

        let mut expected_neighbors: HashMap<u128, SocketAddr> = HashMap::new();
        expected_neighbors.insert(3, "127.0.0.1:6003".parse().unwrap());
        expected_neighbors.insert(1, "127.0.0.1:6001".parse().unwrap());
        expected_neighbors.insert(4, "127.0.0.1:6004".parse().unwrap());

        assert_eq!(
            node.ip_address, expected_ip_address,
            "IP address should be 127.0.0.1:6000"
        );

        assert_eq!(
            node.transfer_speed, expected_transfer_speed,
            "Transfer speed should be 100"
        );

        for (node_id, addr) in expected_neighbors.iter() {
            assert_eq!(
                node.neighbors_hashmap.get(node_id),
                Some(addr),
                "Neighbor {} should map to {}",
                node_id,
                addr
            );
        }
    }
}
