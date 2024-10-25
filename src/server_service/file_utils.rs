use std::collections::HashSet;
use std::path::PathBuf;
use std::env;
use std::fs::{self, read_dir};

pub struct FileUtils {
    node_id: u128,
    folder_path: Box<PathBuf>
}

impl FileUtils {
    pub fn new(node_id: u128) -> Self {
        let folder_name: String = node_id.to_string();
        let folder_path: PathBuf = env::current_dir().unwrap().join(&format!("nodes/{}", folder_name));

        if !folder_path.exists() {
            fs::create_dir_all(&folder_path).expect("Failed to create folder!");
        }

        FileUtils {
            node_id,
            folder_path: folder_path.into()
        }
    }

    pub fn get_chunks_from_folder (&self, file_name: &String) -> Vec<u8> {
        let mut hash_set = HashSet::<u8>::new();

        read_dir(self.folder_path.as_path())
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| {
                let file_path_name = path.file_name().unwrap().to_str().unwrap();
                path.is_file() && file_path_name.starts_with(&format!("{}.ch", file_name.clone()))
            })
            .for_each(|path| {
                let file_path_name = path.file_name().unwrap().to_str().unwrap();
                let path_vec: Vec<&str> = file_path_name.split(&format!("{}.ch", file_name.clone())).collect();
                let chunk_id: u8 = path_vec[1].parse().unwrap();

                hash_set.insert(chunk_id);
            });
        
        let mut result= Vec::<u8>::new(); 
        
        hash_set.iter().for_each(|a| result.push(*a));
        
        result.sort();
        result
    }
}