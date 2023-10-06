use clap::{arg, command, crate_authors, Arg};
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let matches = command!()
        .author(crate_authors!("\n"))
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Shows all fortunes, including unkind."),
        )
        .arg(
            Arg::new("unkind")
                .short('o')
                .short('u')
                .long("unkind")
                .help("Shows only unkind fortunes."),
        )
        .arg(
            Arg::new("find")
                .short('m')
                .long("find")
                .value_name("pattern")
                .help("Finds fortunes matching regex query."),
        )
        .arg(
            Arg::new("length")
                .short('n')
                .long("length")
                .help("Finds a fortune that is shorter than provided number."),
        )
        .arg(arg!(-s --short ... "Shows a short aporism."))
        .get_matches();

    if let Some(pattern) = matches.get_one::<String>("find") {
        fortune::search_fortunes(pattern);
    } else if let Some(short) = matches.get_one::<u8>("short") {
        fortune::get_quote(short);
    } else {
        fortune::get_quote(&0);
    }

    Ok(())
}

pub mod random {
    //! A module for generating random numbers.
    use rand::thread_rng;
    use rand::Rng;

    /// Generates a random number between 0 (inclusive) and the given upper bound (exclusive).
    ///
    /// # Arguments
    ///
    /// * `i` - The upper bound for the random number generation. The generated number will be in the range [0, i).
    ///
    /// # Returns
    ///
    /// A random `usize` number between 0 (inclusive) and `i` (exclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::random::random;
    ///
    /// let num = random(10);
    /// assert!(num < 10);
    /// ```
    pub fn random(i: usize) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(0..i)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Tests that the `random` function generates a number within the expected range.
        #[test]
        fn test_random_within_range() {
            for _ in 0..1000 {
                let upper_bound = 10;
                let num = random(upper_bound);
                assert!(num < upper_bound);
            }
        }

        /// Tests that the `random` function generates a number of 0 when the upper bound is 1.
        #[test]
        fn test_random_upper_bound_one() {
            let num = random(1);
            assert_eq!(num, 0);
        }

        /// Tests that the `random` function panics when the upper bound is 0.
        #[test]
        #[should_panic(expected = "cannot sample empty range")]
        fn test_random_upper_bound_zero() {
            random(0);
        }
    }
}

pub mod fortune {
    //! A module for retrieving random quotes (or fortune).
    use crate::file;
    use crate::random;

    use std::env;
    use std::process::exit;

    /// The default maximum length for a short quote.
    const SHORT: usize = 150;

    /// The default place to look for fortunes
    const FORTUNE_DIR: &str = "fortunes";

    /// The default place to look for off-color fortunes
    const FORTUNE_OFF_DIR: &str = "fortunes_off";

    fn get_fortune_dir() -> String {
        match env::var("FORTUNE_DIR") {
            Ok(val) => val,
            Err(_) => FORTUNE_DIR.to_string(),
        }
    }

    fn get_fortune_off_dir() -> String {
        match env::var("FORTUNE_OFF_DIR") {
            Ok(val) => val,
            Err(_) => FORTUNE_OFF_DIR.to_string(),
        }
    }

    pub fn search_fortunes(pattern: &str) {
        let fortune_dir = get_fortune_dir();

        // TODO: Handle your errors!
        let files = file::read_all_files(&fortune_dir).unwrap();
        for file in files {
            let fortune: Option<&str> = file.split("\n%\n").find(|x| x.contains(pattern));
            if let Some(fortune) = fortune {
                println!("{}\n%", fortune);
            }
        }
    }

    /// Retrieves and prints a random quote from the "fortune" file based on the specified size.
    ///
    /// The function divides the file content into quotes using the "\n%\n" delimiter.
    /// Depending on the `quote_size` parameter, the function will either print a quote of a specific length
    /// or a completely random one.
    ///
    /// # Arguments
    ///
    /// * `quote_size` - A reference to a byte that determines the size of the quote to retrieve.
    ///   - `1`: Default size (up to 150 characters).
    ///   - `2-254`: Reduces the target length by half for each increment.
    ///   - `255`: Prints a humorous message and exits.
    ///   - `0` or any other value: Retrieves a completely random quote.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to pick a file from the "fortune" directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate_name::fortune::get_quote;
    ///
    /// get_quote(&1); // Retrieves a quote of default size.
    /// get_quote(&255); // Prints a humorous message and exits.
    /// ```
    pub fn get_quote(quote_size: &u8) {
        let fortune_dir = get_fortune_dir();

        // FIXME: mom... I'm not feeling so good...
        // please refactor me mother!
        use std::io::ErrorKind;
        let file = match file::pick_file(fortune_dir.clone()) {
            Ok(val) => val,
            Err(e) => {
                if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                    match io_err {
                        err if io_err.kind() == ErrorKind::NotFound => {
                            eprintln!("{err}");
                            println!("Couldn't find \"{fortune_dir}\", make sure you set FORTUNE_DIR correctly, or verify that you're in a directory with a folder named \"{fortune_dir}\".",);
                            std::process::exit(1);
                        }
                        &_ => panic!("{e:?}"),
                    }
                }
                panic!("{e:?}")
            }
        };
        let quotes: Vec<&str> = file.split("\n%\n").collect();

        let mut tmp = vec![];

        match quote_size {
            n if n > &0 => {
                let mut target_length: usize = SHORT;
                if *n != 1 {
                    for _ in 1..*n {
                        target_length /= 2;
                    }
                }
                if *n == 255_u8 {
                    println!("WE GET IT, YOU WANT A SHORT FORTUNE");
                    exit(0);
                }
                if target_length < 1 {
                    target_length = 1;
                }
                println!("{n}");
                for q in &quotes {
                    if q.len() <= target_length {
                        tmp.push(q)
                    }
                }
                println!("{}", tmp[random::random(tmp.len())]);
            }
            _ => {
                println!("{}", quotes[random::random(quotes.len() - 1)]);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use assert_cmd::Command;

        /// Tests the behavior of `get_quote` when the default size (1) is provided.
        /// It ensures that the output quote is within the expected length.
        #[test]
        fn test_get_quote_default_size() {
            let mut cmd = Command::cargo_bin("fortune-kind").unwrap();
            cmd.arg("-s");
            let output = cmd.output().unwrap();
            assert!(output.stdout.len() <= SHORT as usize);
        }

        /// Tests the behavior of `get_quote` when the humorous message trigger (255) is provided.
        /// It ensures that the output matches the expected humorous message.
        #[test]
        fn test_get_quote_humorous_message() {
            let mut cmd = Command::cargo_bin("fortune-kind").unwrap();
            cmd.arg(format!("-{}", String::from("s").repeat(255)));
            let output = cmd.output().unwrap();
            assert_eq!(output.stdout, b"WE GET IT, YOU WANT A SHORT FORTUNE\n");
        }
    }
}

pub mod file {
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
}
