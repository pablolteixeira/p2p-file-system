use std::{env, fs::{self, read_dir, File}, io::{BufWriter, Read, Write}, path::{self, PathBuf}};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    file_name: String,
}

struct ChunkCreator {
    file_path: Box<PathBuf>,
    chunks_amount: u8
}

impl ChunkCreator {
    fn new(file_path: &String, chunks_amount: u8) -> Self {
        let file_path = PathBuf::from(&file_path);
        let mut current_dir = env::current_dir().unwrap();

        current_dir.push(file_path);
        let absolute_path = path::absolute(&current_dir).unwrap();

        ChunkCreator {
            file_path: absolute_path.into(),
            chunks_amount
        }
    }

    fn create_chunks(&self) {
        let mut file = File::open(self.file_path.as_path()).unwrap();
        let mut data: Vec<u8> = Vec::new();

        let _ = file.read_to_end(&mut data); 
        
        let file_name = self
            .file_path
            .as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let folder_path = format!("files/{}-chunks", file_name);
        let chunks_folder_path = PathBuf::from(folder_path);
        let _ = fs::create_dir(chunks_folder_path.clone());

        let chunk_size = (data.len() as f64 / self.chunks_amount as f64).ceil() as usize;

        for i in 0..self.chunks_amount {
            let chunk_file_name = format!("{}.ch{}", file_name, i);
            let chunk_file_path = chunks_folder_path.join(chunk_file_name);

            let mut buffer = BufWriter::new(File::create_new(chunk_file_path).unwrap());

            let start = i as usize * chunk_size;
            let end = ((i + 1) as usize * chunk_size).min(data.len());

            buffer.write(&data[start..end]).unwrap();

            buffer.flush().unwrap();
        }
    }

    fn read_chunks(file_name: String) {
        let folder_name = format!("files/{}-chunks", file_name.clone());
        let mut current_dir = env::current_dir().unwrap();
        current_dir.push(folder_name);
        let absolute_path = path::absolute(&current_dir).unwrap();
        
        let mut entries: Vec<PathBuf> = read_dir(absolute_path)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| {
                let file_path_name = path.file_name().unwrap().to_str().unwrap();
                path.is_file() && file_path_name.starts_with(&format!("{}.ch", file_name.clone()))
            })
            .collect();

        entries.sort_by_key(|path| {
            let file_path_name = path.file_name().unwrap().to_str().unwrap();
            file_path_name
                .split(".ch")
                .nth(1)
                .and_then(|index| index.parse::<u8>().ok())
                .unwrap_or(0)
        });

        let mut new_buffer = BufWriter::new(File::create(format!("new-{}", file_name)).unwrap());

        for path in entries {
            let mut file = File::open(&path).unwrap();
            let mut data: Vec<u8> = Vec::new();
            let _ = file.read_to_end(&mut data);

            new_buffer.write(&data).unwrap();
        }

        new_buffer.flush().unwrap();
    }
}

fn main () {
    let args: Cli = Cli::parse();

    println!("Create chunks!");

    let file_path = format!("files/{}", args.file_name);

    let chunk_creator = ChunkCreator::new(&file_path, 10);
    chunk_creator.create_chunks();

    ChunkCreator::read_chunks(args.file_name);
}