//! A module for file related actions.
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

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

/// Retrieves the sizes of files in the specified directory.
///
/// This function will traverse the directory given by `path` and return a vector
/// of tuples. Each tuple contains the file size in bytes and the path to the file as `PathBuf`.
///
/// # Arguments
///
/// * `path` - A generic parameter that implements `AsRef<Path>`, which is the path to the directory to read.
///
/// # Returns
///
/// A `std::io::Result` containing a vector of tuples. Each tuple consists of a `u64` file size
/// and a `PathBuf` corresponding to a file's path. If an error occurs during directory traversal
/// or metadata retrieval, an `io::Error` is returned.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// * The path does not exist.
/// * The current process lacks permissions to read the directory.
/// * The path points to a non-directory file.
/// * Any I/O error encountered when reading the directory contents or retrieving file metadata.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let sizes = get_file_sizes(Path::new("./some/directory")).expect("Directory should be read");
/// for (size, path) in sizes {
///     println!("{} bytes - {:?}", size, path);
/// }
/// ```
pub fn get_file_sizes<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<(u64, PathBuf)>> {
    let entries = fs::read_dir(path)?;
    let mut files: Vec<(u64, PathBuf)> = vec![];

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            files.push((metadata.len(), path));
        }
    }

    Ok(files)
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

    /// test_read_all_files_invalid_dir: Tests the error handling of read_all_files when given an invalid directory.
    #[test]
    fn test_read_all_files_invalid_dir() {
        let result = read_all_files("invalid_directory");
        assert!(result.is_err());
    }
}
