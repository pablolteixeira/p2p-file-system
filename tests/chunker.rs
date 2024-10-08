use std::fs::File;
use std::io::{self, Read, Write};
use tempfile::tempdir;
use std::path::Path;

use p2p_file_system::dto::chunker::Chunker;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom};

    #[test]
    fn test_chunker_reconstruction() -> io::Result<()> {
        // Create a temporary directory
        let dir = tempdir()?;
        let original_file_path = dir.path().join("original.txt");
        let reconstructed_file_path = dir.path().join("reconstructed.txt");

        // Write some content to the original file
        let original_content = b"Hello, this is a test file to be chunked and reconstructed!";
        let mut original_file = File::create(&original_file_path)?;
        original_file.write_all(original_content)?;
        original_file.sync_all()?;

        // Initialize the chunker
        let amount_of_chunks = 4;
        let mut chunker = Chunker::new(original_file_path.to_str().unwrap(), amount_of_chunks)?;

        // Get all chunks from the file
        let mut chunks = Vec::new();
        for i in 0..amount_of_chunks {
            chunks.push(chunker.get_chunk(i)?);
        }

        // Reconstruct the file from the chunks
        Chunker::construct_file(chunks, reconstructed_file_path.to_str().unwrap())?;

        // Read the reconstructed file and compare with the original content
        let mut reconstructed_file = File::open(reconstructed_file_path)?;
        let mut reconstructed_content = Vec::new();
        reconstructed_file.read_to_end(&mut reconstructed_content)?;

        // Assert the reconstructed content matches the original content
        assert_eq!(reconstructed_content, original_content, "The reconstructed file content should match the original");

        Ok(())
    }
}
