use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::env;

pub struct Descritor {
    node_id: u128,
    config_path: Box<Path>,
    topology_path: Box<Path>,
    node_folder: Box<Path>
}

impl Descritor {
    pub fn new(node_id: u128) -> Self {
        let config_path = Path::new("config/config.txt");
        let topology_path = Path::new("config/topologia.txt");
        
        let folder_name = format!("nodes/{}", node_id);
        
        let node_folder = Path::new(&folder_name);

        Descritor {
            node_id,
            config_path: config_path.into(),
            topology_path: topology_path.into(),
            node_folder: node_folder.into()
        }
    }

    pub fn parse_file(&self) -> (SocketAddr, u32, HashMap<u128, SocketAddr>) {
        let topology_file = File::open(&self.topology_path).expect("Should have been able to read the file");
        let topology_reader = io::BufReader::new(topology_file);

        let mut neighbors_hashset = HashSet::<u128>::new(); 

        for line in topology_reader.lines() {
            match line {
               Ok(line) => {
                    let line_splitted: Vec<&str> = line.split(" ").collect();
                    let node_topology_id = line_splitted[0].strip_suffix(":").unwrap().parse().unwrap();

                    if self.node_id == node_topology_id {
                        for item in line_splitted[1..].to_vec() {
                            let neighbor_id: u128 = item.trim_end_matches(",").parse().unwrap();

                            neighbors_hashset.insert(neighbor_id);
                        }

                        break;
                    }
               },
               Err(_) => {} 
            }
        }

        let config_file = File::open(&self.config_path).expect("Should have been able to read the file");
        let config_reader = io::BufReader::new(config_file);

        let mut hash_map: HashMap<u128, SocketAddr> = HashMap::<u128, SocketAddr>::new();
        let mut ip_address: Option<SocketAddr> = None;
        let mut transfer_speed: u32 = 100;

        for line in config_reader.lines() {
            match line {
               Ok(line) => {
                    let line_splitted: Vec<&str> = line.split(" ").collect();

                    let node_config_id: u128 = line_splitted[0].strip_suffix(":").unwrap().parse().unwrap();
                    
                    let address = line_splitted[1].strip_suffix(",").unwrap();
                    let port = line_splitted[2].strip_suffix(",").unwrap();

                    let ip_adrress: SocketAddr = format!("{}:{}", address, port).parse().unwrap();

                    if self.node_id != node_config_id {
                        if neighbors_hashset.contains(&node_config_id) {
                            hash_map.insert(node_config_id, ip_adrress);
                        }
                    } else {
                        let ip_address_socket: SocketAddr = format!("{}:{}", address, port).parse().unwrap();
                        ip_address = Some(ip_address_socket);
                        transfer_speed = line_splitted[3].parse().unwrap();
                    }
               },
               Err(_) => {} 
            }
        }

        return (ip_address.unwrap(), transfer_speed, hash_map);
    }
}