use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

pub struct Chunker {
    file: File,
    amount_of_chunks: u32,
    file_size: u64,
}

impl Chunker {
    pub fn new(file_path: &str, amount_of_chunks: u32) -> io::Result<Chunker> {
        let file = File::open(file_path)?;

        let mut file_size = file.metadata()?.len() as u64;

        Ok(Chunker {file, amount_of_chunks, file_size })
    }

    pub fn get_chunk(&mut self, chunk_position: u32) -> io::Result<Vec<u8>> {

        if chunk_position >= self.amount_of_chunks {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Chunk position out of bounds",
            ));
        }

        let chunk_size = self.file_size/self.amount_of_chunks as u64;
        let remainder = self.file_size % self.amount_of_chunks as u64;

        if chunk_position == self.amount_of_chunks - 1 {
            let chunk_size = chunk_size + remainder;
        }

        let mut buffer = vec![0; chunk_size as usize];

        self.file.seek(SeekFrom::Start((chunk_position as u64 * chunk_size)))?;

        self.file.read_exact(&mut buffer)?;
        Ok(buffer)

    }
}

