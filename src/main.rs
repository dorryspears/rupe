use clap::Parser;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "A program to find duplicate files in a directory"
)]
struct Args {
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("Path: {}!", args.path);

    let path = Path::new(&args.path);
    if !path.exists() {
        println!("Path does not exist!");
        return Ok(());
    }

    if !path.is_dir() {
        println!("Path is not a directory!");
        return Ok(());
    }

    let mut files: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for entry in path.read_dir()? {
        if let Ok(entry) = entry {
            if entry.metadata()?.is_file() {
                let file_size = entry.metadata()?.len();
                if files.contains_key(&file_size) {
                    files.get_mut(&file_size).unwrap().push(entry.path());
                } else {
                    files.insert(file_size, vec![entry.path()]);
                }
            }
        } else {
            println!("Error reading file");
        }
    }

    let mut keys_to_remove: Vec<u64> = Vec::new();
    for (key, value) in &files {
        if value.len() == 1 {
            keys_to_remove.push(*key);
        }
    }

    for key in keys_to_remove {
        files.remove(&key);
    }

    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (_key, files_with_same_size) in &files {
        let mut temp_hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for file in files_with_same_size {
            let hash = hash_file(&file)?;

            if let Some(files_with_same_hash) = temp_hash_map.get_mut(&hash) {
                files_with_same_hash.push(file.clone());
            } else {
                temp_hash_map.insert(hash, vec![file.clone()]);
            }
        }

        for (hash, files_with_same_hash) in temp_hash_map {
            if files_with_same_hash.len() > 1 {
                if let Some(files_in_hash_map) = hash_map.get_mut(&hash) {
                    files_in_hash_map.extend(files_with_same_hash);
                } else {
                    hash_map.insert(hash, files_with_same_hash);
                }
            }
        }
    }

    // Print the main hash map
    for (hash, files) in &hash_map {
        println!("{}: {:?}", hash, files);
    }

    Ok(())
}

pub fn hash_file(file: &PathBuf) -> io::Result<String> {
    let mut file = File::open(file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.input(&buffer);
    Ok(hasher.result_str())
}
