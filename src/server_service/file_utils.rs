use std::collections::{HashMap, HashSet};
use std::{fs};
use std::path::{Path, PathBuf};
use std::env;
use std::fmt::Debug;
use std::fs::{File, ReadDir};
use std::io::Read;

pub struct FileUtils {
    node_id: u128,
    chunk_table: HashMap<String, HashSet<u64>>,
}

impl FileUtils {
    pub fn new(node_id: u128) -> Self {
        let mut chunk_table = HashMap::new();
        Self{
            node_id, chunk_table
        }
    }

    pub fn startup (&mut self) {
        let folder_name: String = self.node_id.to_string();
        let folder_path:PathBuf = env::current_dir().unwrap().join(&folder_name);
        let path:&Path = Path::new(&folder_path);

        if path.exists() {
            self.get_files_from_folder(path);
        }
        else 
        {
            fs::create_dir_all(&folder_path).expect("Failed to create folder!");
        }
    }

    pub fn get_files_from_folder (&mut self, path: &Path){
        let dir_structure:ReadDir  = fs::read_dir(path).unwrap();

        for entry in dir_structure {
            let file_name:String = entry.unwrap().path().file_name().unwrap().to_str().unwrap().to_string();
            let partitioned_name = file_name.split(".ch").collect::<Vec<&str>>();

            if partitioned_name.len() > 1 {
                let base_file_name = partitioned_name[0].to_string();
                let chunk_number = partitioned_name[1].parse::<u64>().expect("Failed to parse chunk number!");
                self.chunk_table.entry(base_file_name).or_insert_with(HashSet::new).insert(chunk_number);
            } else {
                self.chunk_table.entry(partitioned_name[0].to_string()).or_insert_with(HashSet::new).insert(0);

            }

        }
    }

    pub fn add_file_to_table(&mut self, file_name: String, chunk_number: u64){
        self.chunk_table.entry(file_name).or_insert_with(HashSet::new).insert(chunk_number);
    }

    pub fn get_chunks_from_file (&mut self, file_name: String) -> &HashSet<u64> {
        self.chunk_table.get(&file_name).unwrap()
    }

    pub fn read_file(&self, file_name: &str, chunk_number: u64) -> Vec<u8> {
        let full_file_name = format!("{}.ch{}", file_name, chunk_number);

        let folder_name: String = self.node_id.to_string();
        let folder_path: PathBuf = env::current_dir().unwrap().join(&folder_name);
        let file_path = folder_path.join(full_file_name);

        let mut file = File::open(&file_path).expect("Failed to open file!");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file!");

        buffer
    }
}