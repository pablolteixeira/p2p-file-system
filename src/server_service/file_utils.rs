use std::{
    collections::HashSet,
    env,
    fs::{self, read_dir, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::dto::chunk_data::ChunkData;

/// Utility class for file operations related to chunks.
pub struct FileUtils {
    node_id: u128,
    folder_path: PathBuf,
}

impl FileUtils {
    /// Creates a new `FileUtils` instance.
    pub fn new(node_id: u128) -> Self {
        let folder_name = node_id.to_string();
        let folder_path = env::current_dir()
            .unwrap()
            .join(&format!("nodes/{}", folder_name));

        if !folder_path.exists() {
            fs::create_dir_all(&folder_path).expect("Failed to create folder!");
        }

        FileUtils {
            node_id,
            folder_path,
        }
    }

    /// Retrieves the list of chunks available in the folder for a given file.
    pub fn get_chunks_from_folder(&self, file_name: &str) -> Vec<u8> {
        let mut hash_set = HashSet::<u8>::new();

        read_dir(&self.folder_path)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| {
                let file_path_name = path.file_name().unwrap().to_str().unwrap();
                path.is_file() && file_path_name.starts_with(&format!("{}.ch", file_name))
            })
            .for_each(|path| {
                let file_path_name = path.file_name().unwrap().to_str().unwrap();
                let path_vec: Vec<&str> =
                    file_path_name.split(&format!("{}.ch", file_name)).collect();
                if let Some(chunk_id_str) = path_vec.get(1) {
                    if let Ok(chunk_id) = chunk_id_str.parse::<u8>() {
                        hash_set.insert(chunk_id);
                    }
                }
            });

        let mut result = Vec::<u8>::new();

        hash_set.iter().for_each(|a| result.push(*a));

        result.sort();
        result
    }

    /// Retrieves the data for the specified chunks of a file.
    pub fn get_chunks_data(&self, file_name: &str, chunks: &[u8]) -> Vec<ChunkData> {
        let mut chunk_datas = Vec::new();
        for &chunk_id in chunks {
            if let Some(chunk_data) = self.read_chunk(file_name, chunk_id) {
                chunk_datas.push(ChunkData {
                    chunk_id,
                    data: chunk_data,
                });
            }
        }
        chunk_datas
    }

    /// Reads a specific chunk from the file system.
    pub fn read_chunk(&self, file_name: &str, chunk_id: u8) -> Option<Vec<u8>> {
        let chunk_file_name = format!("{}.ch{}", file_name, chunk_id);
        let path = self.folder_path.join(chunk_file_name);
        if path.exists() {
            let mut file = File::open(&path).ok()?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).ok()?;
            Some(buffer)
        } else {
            None
        }
    }

    /// Saves received chunks to the file system.
    pub fn save_chunks(&self, file_name: &str, chunk_datas: &[ChunkData]) {
        let mut sorted_chunks = chunk_datas.to_vec();
        sorted_chunks.sort_by_key(|cd| cd.chunk_id);

        let path = self.folder_path.join(file_name);
        let mut file = File::create(&path).expect("Failed to create file");

        // Write each chunk's data to the file in order
        for chunk_data in sorted_chunks {
            file.write_all(&chunk_data.data)
                .expect("Failed to write chunk data to file");
        }

        println!("Saved file {}", path.display());
    }
}
