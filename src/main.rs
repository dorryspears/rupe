mod utils;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use utils::{find_duplicate_hashes, find_files_with_same_size, hash_file};


#[derive(Parser, Debug)]
#[clap(
    author = "Dorry",
    version = "0.2.0",
    about = "A program to find duplicate files in a directory"
)]
struct Args {
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Finding duplicates in folder: {}!", args.path);

    let path = Path::new(&args.path);
    if !path.exists() {
        println!("Path does not exist!");
        return Ok(());
    }

    if !path.is_dir() {
        println!("Path is not a directory!");
        return Ok(());
    }

    let files = find_files_with_same_size(path)?;

    let mut duplicate_files: Vec<String> = Vec::new();

    for (_key, files_with_same_size) in &files {
        
        let mut bucket_hashes: HashMap<String, Vec<PathBuf>> = HashMap::new();

        let duplicate_hashes = find_duplicate_hashes(files_with_same_size)?;

        for file in files_with_same_size {
            let hash = hash_file(file)?;
            if duplicate_hashes.contains(&hash) {
                if bucket_hashes.contains_key(&hash) {
                    bucket_hashes.get_mut(&hash).unwrap().push(file.clone());
                } else {
                    bucket_hashes.insert(hash, vec![file.clone()]);
                }
            }
        }

        // Binary comparison for each file in the bucket
        for (_hash, files) in &bucket_hashes {
            for i in 0..files.len() {
                for j in i+1..files.len() {
                    let mut file1 = File::open(&files[i])?;
                    let mut file2 = File::open(&files[j])?;
                    let mut buffer1 = Vec::new();
                    let mut buffer2 = Vec::new();
                    file1.read_to_end(&mut buffer1)?;
                    file2.read_to_end(&mut buffer2)?;
                    if buffer1 == buffer2 {
                        duplicate_files.push(format!("{} = {}", files[i].to_str().unwrap(), files[j].to_str().unwrap()));
                    }
                }
            }
        } 
    }

    for file in duplicate_files {
        println!("{}", file);
    }

    Ok(())
}

