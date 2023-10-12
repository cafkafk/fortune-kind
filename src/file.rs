//! A module for file related actions.
use rand::prelude::SliceRandom;
use std::fs;
use std::io::Read;

/// Picks a random file from a given directory and returns its contents as a string.
///
/// # Arguments
///
/// * `dir` - The path to the directory from which a random file will be chosen.
///
/// # Returns
///
/// A `Result` containing the contents of the randomly chosen file as a `String`.
///
/// # Errors
///
/// Returns an error if:
/// * The provided directory path is invalid or inaccessible.
/// * No files are found in the specified directory.
/// * There's an issue reading the chosen file.
///
/// # Note
///
/// This function does not account for the number of fortunes (or entries) within the files.
pub fn pick_file(dir: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let files: Vec<_> = fs::read_dir(dir)?.collect();
    let file = files.choose(&mut rng).ok_or("No files found")?;
    let path = file.as_ref().unwrap().path();

    let mut contents = String::new();
    fs::File::open(path)?.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Reads the contents of all files in a given directory and returns them as a vector of strings.
///
/// # Arguments
///
/// * `dir` - The path to the directory from which all files will be read.
///
/// # Returns
///
/// A `Result` containing a vector of strings, where each string represents the contents of a file in the directory.
///
/// # Errors
///
/// Returns an error if:
/// * The provided directory path is invalid or inaccessible.
/// * There's an issue reading any of the files.
pub fn read_all_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir)?;
    let mut contents_vec = Vec::new();

    for entry in entries {
        let path = entry?.path();
        if path.is_file() {
            let mut contents = String::new();
            fs::File::open(path)?.read_to_string(&mut contents)?;
            contents_vec.push(contents);
        }
    }

    Ok(contents_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    // Helper function to create a temporary directory with some files for testing.
    fn setup_test_directory() -> tempfile::TempDir {
        let tmp_dir = tempfile::tempdir().unwrap();
        let file1_path = tmp_dir.path().join("file1.txt");
        let file2_path = tmp_dir.path().join("file2.txt");

        let mut file1 = File::create(file1_path).unwrap();
        let mut file2 = File::create(file2_path).unwrap();

        writeln!(file1, "Content of file1").unwrap();
        writeln!(file2, "Content of file2").unwrap();

        tmp_dir
    }

    /// test_pick_file: Tests if the pick_file function can pick and read a file from a directory.
    #[test]
    fn test_pick_file() {
        let tmp_dir = setup_test_directory();
        let result = pick_file(tmp_dir.path().to_str().unwrap().to_string());

        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content == "Content of file1\n" || content == "Content of file2\n");
    }

    /// test_read_all_files: Tests if the read_all_files function can read all files from a directory.
    #[test]
    fn test_read_all_files() {
        let tmp_dir = setup_test_directory();
        let result = read_all_files(tmp_dir.path().to_str().unwrap());

        assert!(result.is_ok());
        let contents = result.unwrap();
        assert_eq!(contents.len(), 2);
        assert!(contents.contains(&"Content of file1\n".to_string()));
        assert!(contents.contains(&"Content of file2\n".to_string()));
    }

    /// test_pick_file_invalid_dir: Tests the error handling of pick_file when given an invalid directory.
    #[test]
    fn test_pick_file_invalid_dir() {
        let result = pick_file("invalid_directory".to_string());
        assert!(result.is_err());
    }

    /// test_read_all_files_invalid_dir: Tests the error handling of read_all_files when given an invalid directory.
    #[test]
    fn test_read_all_files_invalid_dir() {
        let result = read_all_files("invalid_directory");
        assert!(result.is_err());
    }
}
