use crate::hash_file;

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use std::fs::File;
    use tempfile::tempdir;


    #[test]
    fn test_hash_file() -> io::Result<()> {
        // Create a temporary directory
        let dir = tempdir()?;

        // Create a file in the temporary directory
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "Hello, world!")?;

        // Hash the file
        let hash = hash_file(&file_path)?;

        // Check the hash
        assert_eq!(hash, "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b");

        // Delete the temporary directory
        dir.close()?;

        Ok(())
    }
}