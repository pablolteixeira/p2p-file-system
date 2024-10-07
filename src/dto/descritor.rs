use std::collections::{HashMap, HashSet};
use std::fs::{create_dir, remove_dir_all, File};
use std::io::{self, BufRead, BufReader};
use std::net::SocketAddr;
use std::path::Path;

pub struct Descritor {
    node_id: u128,
    config_path: Box<Path>,
    topology_path: Box<Path>,
    node_folder: Box<Path>,
}

impl Descritor {
    pub fn new(node_id: u128) -> Self {
        let config_path: &Path = Path::new("config/config.txt");
        let topology_path: &Path = Path::new("config/topologia.txt");

        let folder_name: String = format!("nodes/{}", node_id);
        let node_folder: &Path = Path::new(&folder_name);

        let _ = create_dir(node_folder);

        Descritor {
            node_id,
            config_path: config_path.into(),
            topology_path: topology_path.into(),
            node_folder: node_folder.into(),
        }
    }

    pub fn parse_file(&self) -> (SocketAddr, u32, HashMap<u128, SocketAddr>) {
        let neighbors_hashset: HashSet<u128> = self.parse_topology_file();
        return self.parse_config_file(neighbors_hashset);
    }

    fn parse_topology_file(&self) -> HashSet<u128> {
        let topology_file: File =
            File::open(&self.topology_path).expect("Should have been able to read the file");
        let topology_reader: BufReader<File> = io::BufReader::new(topology_file);
        let mut neighbors_hashset: HashSet<u128> = HashSet::<u128>::new();

        for line in topology_reader.lines() {
            if let Ok(line) = line {
                let line_splitted: Vec<&str> = line.split_whitespace().collect();
                let node_topology_id: u128 = line_splitted[0]
                    .trim_end_matches(":")
                    .parse::<u128>()
                    .unwrap();
                if self.node_id == node_topology_id {
                    for item in &line_splitted[1..] {
                        let neighbor_id: u128 = item.trim_end_matches(",").parse::<u128>().unwrap();

                        neighbors_hashset.insert(neighbor_id);
                    }
                    break;
                }
            }
        }
        neighbors_hashset
    }

    fn parse_config_file(
        &self,
        neighbors_hashset: HashSet<u128>,
    ) -> (SocketAddr, u32, HashMap<u128, SocketAddr>) {
        let config_file: File =
            File::open(&self.config_path).expect("Should have been able to read the config file");
        let config_reader: BufReader<File> = io::BufReader::new(config_file);

        let mut hash_map: HashMap<u128, SocketAddr> = HashMap::<u128, SocketAddr>::new();
        let mut ip_address: Option<SocketAddr> = None;
        let mut transfer_speed: u32 = 100;

        for line in config_reader.lines() {
            if let Ok(line) = line {
                let line_splitted: Vec<&str> = line.split_whitespace().collect();
                let node_config_id: u128 = line_splitted[0]
                    .trim_end_matches(":")
                    .parse::<u128>()
                    .unwrap();
                let address: &str = line_splitted[1].trim_end_matches(",");
                let port: &str = line_splitted[2].trim_end_matches(",");
                let ip_address_socket: SocketAddr =
                    format!("{}:{}", address, port).parse().unwrap();

                if self.node_id != node_config_id {
                    if neighbors_hashset.contains(&node_config_id) {
                        hash_map.insert(node_config_id, ip_address_socket);
                    }
                } else {
                    ip_address = Some(ip_address_socket);
                    transfer_speed = line_splitted[3].parse().unwrap();
                }
            }
        }

        return (ip_address.unwrap(), transfer_speed, hash_map);
    }
}
