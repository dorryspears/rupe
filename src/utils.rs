use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn find_duplicate_hashes(files_with_same_size: &Vec<PathBuf>) -> Result<HashSet<String>, io::Error> {
    let mut seen_hashes: HashSet<String> = HashSet::new();
    let mut duplicate_hashes: HashSet<String> = HashSet::new();
    for file in files_with_same_size {
        let hash = hash_file(file)?;
        if seen_hashes.contains(&hash) {
            duplicate_hashes.insert(hash);
        } else {
            seen_hashes.insert(hash);
        }
    }
    Ok(duplicate_hashes)
}

/// Find all files in a directory with the same size
pub fn find_files_with_same_size(path: &Path) -> Result<HashMap<u64, Vec<PathBuf>>, io::Error> {
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

    // Remove files with only one file
    let mut keys_to_remove: Vec<u64> = Vec::new();
    for (key, value) in &files {
        if value.len() == 1 {
            keys_to_remove.push(*key);
        }
    }

    for key in keys_to_remove {
        files.remove(&key);
    }

    Ok(files)
}

pub fn hash_file(file: &PathBuf) -> io::Result<String> {
    let mut file = File::open(file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.input(&buffer);
    Ok(hasher.result_str())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use std::io;

    #[test]
    fn test_hash_file() -> io::Result<()> {
        let path = PathBuf::from("test.txt");
        let mut file = File::create(&path)?;
        file.write_all(b"Hello World!")?;
        let hash = hash_file(&path)?;
        assert_eq!(hash, "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069");
        let _ = fs::remove_file(&path); // Remove the file
        Ok(())
    }

    #[test]
    fn test_find_files_with_same_size_return_none() -> io::Result<()> {
        let path = PathBuf::from("test.txt");
        let mut file = File::create(&path)?;
        file.write_all(b"Hello World!")?;
        let files = find_files_with_same_size(Path::new("."))?;
        assert_eq!(files.len(), 0);
        let _ = fs::remove_file(&path); // Remove the file
        Ok(())
    }
}
