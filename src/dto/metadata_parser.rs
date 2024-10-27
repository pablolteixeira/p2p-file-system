use std::{fs::File, io::{self, BufRead, BufReader}, path::Path, sync::Arc};

pub struct MetadataParser {
    metadata_file_path: Arc<Path>
}

impl MetadataParser {
    pub fn new(file_path: &str) -> Self {
        let metadata_file_path = Path::new(&file_path);

        MetadataParser {
            metadata_file_path: metadata_file_path.into()
        }
    }

    pub fn parse(&self) -> (String, u8, u32) {
        let metadata_file: File = File::open(&self.metadata_file_path).expect("Should have been able to read the file");
    
        let metadata_file_reader: BufReader<File> = io::BufReader::new(metadata_file);

        let mut file_name = String::new();
        let mut chunks: u8 = 0;
        let mut ttl: u32 = 0;

        for (i, metadata_file_line) in metadata_file_reader.lines().enumerate() {
            if let Ok(metadata_file_line) = metadata_file_line {
                if i == 0 {
                    file_name = metadata_file_line.trim().to_string();
                } else if i == 1 {
                    chunks = metadata_file_line.trim().parse().unwrap();
                } else if i == 2 {
                    ttl = metadata_file_line.trim().parse().unwrap();
                }
            }
        }

        (file_name, chunks, ttl)
    }
}
