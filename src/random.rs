// SPDX-FileCopyrightText: 2023 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: AGPL-3.0-only

//! A module for generating random numbers.
use std::path::PathBuf;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::io::Read;

use crate::file::get_file_sizes;

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

pub fn get_random_file_weighted(path: PathBuf) -> std::io::Result<String> {
    use std::io::ErrorKind;
    let mut rng = thread_rng();
    match get_file_sizes(&path) {
        Ok(mut files) => {
            files.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let mut contents = String::new();
            std::fs::File::open(&files.choose_weighted(&mut rng, |item| item.0).unwrap().1)?
                .read_to_string(&mut contents)?;
            Ok(contents)
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("{e}");
                println!("Couldn't find \"{path:?}\", make sure you set FORTUNE_DIR correctly, or verify that you're in a directory with a folder named \"{path:?}\".",);
                std::process::exit(1);
            }
            _ => panic!("Error: {}", e),
        },
    }
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
