use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
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


        let chunk_size = self.file_size / self.amount_of_chunks as u64;
        let remainder = self.file_size % self.amount_of_chunks as u64;
        let mut buffer = vec![0; chunk_size as usize];
        if chunk_position == self.amount_of_chunks - 1{
            let final_chunk_size = chunk_size + remainder;
            buffer = vec![0; final_chunk_size as usize];
        }



        let seek_position = chunk_position as u64 * chunk_size;
        self.file.seek(SeekFrom::Start(seek_position))?;

        if chunk_position == self.amount_of_chunks - 1 {
            // Calculate the remaining bytes and read exactly that
            let remaining_bytes = self.file_size - seek_position;
            self.file.read_exact(&mut buffer[..remaining_bytes as usize])?;
        } else {
            // For non-last chunks, read the entire chunk size
            self.file.read_exact(&mut buffer)?;
        }

        Ok(buffer)

    }

    pub fn construct_file(file_bytes: Vec<Vec<u8>>, file_path: &str) -> io::Result<()> {
        let output_file = File::create(file_path)?;
        let mut output_file = io::BufWriter::new(output_file);

        for chunk in file_bytes {
            output_file.write_all(&chunk)?;

        }

        Ok(())
    }
}

