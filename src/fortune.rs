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
#[allow(dead_code)] // TODO: remove
const FORTUNE_OFF_DIR: &str = "fortunes_off";

fn get_fortune_dir() -> String {
    match env::var("FORTUNE_DIR") {
        Ok(val) => val,
        Err(_) => FORTUNE_DIR.to_string(),
    }
}

#[allow(dead_code)] // TODO: remove
fn get_fortune_off_dir() -> String {
    match env::var("FORTUNE_OFF_DIR") {
        Ok(val) => val,
        Err(_) => FORTUNE_OFF_DIR.to_string(),
    }
}

// TODO: refactor
fn handle_file_errors(
    input: String,
    f: &dyn Fn(String) -> Result<String, Box<dyn std::error::Error>>,
) -> String {
    use std::io::ErrorKind;
    match f(input.clone()) {
        Ok(val) => val,
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                match io_err {
                    err if io_err.kind() == ErrorKind::NotFound => {
                        eprintln!("{err}");
                        println!("Couldn't find \"{input}\", make sure you set FORTUNE_DIR correctly, or verify that you're in a directory with a folder named \"{input}\".",);
                        std::process::exit(1);
                    }
                    &_ => panic!("{e:?}"),
                }
            }
            panic!("{e:?}")
        }
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

    let file = handle_file_errors(fortune_dir, &file::pick_file);

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

    // /// Tests the behavior of `get_quote` when the default size (1) is provided.
    // /// It ensures that the output quote is within the expected length.
    /* Doesn't work in CI
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
    }*/
}
