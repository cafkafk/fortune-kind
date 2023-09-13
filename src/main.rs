use clap::{arg, command, crate_authors, Arg};
use rand::thread_rng;
use rand::Rng;
use std::env;
use std::io;
use std::process::exit;

const SHORT: usize = 150;

pub mod search {
    //! A module for searching patterns within strings using regular expressions.
    use std::error::Error;

    use grep_matcher::Matcher;
    use grep_regex::RegexMatcher;
    use grep_searcher::sinks::UTF8;
    use grep_searcher::Searcher;

    const SHERLOCK: &'static [u8] = b"\
    For the Doctor Watsons of this world, as opposed to the Sherlock
    Holmeses, success in the province of detective work must always
    be, to a very large extent, the result of luck. Sherlock Holmes
    can extract a clew from a wisp of straw or a flake of cigar ash;
    but Doctor Watson has to have it taken out for him and dusted,
    and exhibited clearly, with a label attached.
    ";

    pub fn example() -> Result<String, Box<dyn Error>> {
        let matcher = RegexMatcher::new(r"Doctor \w+")?;
        let mut matches: Vec<(u64, String)> = vec![];
        Searcher::new().search_slice(
            &matcher,
            SHERLOCK,
            UTF8(|lnum, line| {
                // We are guaranteed to find a match, so the unwrap is OK.
                let mymatch = matcher.find(line.as_bytes())?.unwrap();
                matches.push((lnum, line[mymatch].to_string()));
                Ok(true)
            }),
        )?;

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], (1, "Doctor Watsons".to_string()));
        assert_eq!(matches[1], (5, "Doctor Watson".to_string()));
        Ok(format!("{:#?}", matches))
    }

    /// Searches for a given pattern within an input string and returns the line
    /// number and matched string.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to search within, represented as a byte slice.
    /// * `pattern` - The pattern to search for, represented as a string slice.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of tuples. Each tuple contains:
    /// * A `u64` representing the line number where the match was found.
    /// * A `String` representing the matched string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The provided pattern is not a valid regular expression.
    /// * There's an issue during the search operation.
    pub fn search_string(
        input: &[u8],
        pattern: &str,
    ) -> Result<Vec<(u64, String)>, Box<dyn Error>> {
        let matcher = RegexMatcher::new(pattern)?;
        let mut matches: Vec<(u64, String)> = vec![];
        Searcher::new().search_slice(
            &matcher,
            input,
            UTF8(|lnum, line| {
                let mymatch = matcher.find(line.as_bytes())?.unwrap();
                matches.push((lnum, line[mymatch].to_string()));
                Ok(true)
            }),
        )?;

        Ok(matches)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Tests the `search_string` function with a simple pattern.
        #[test]
        fn search_string_simple() {
            let res = search_string(
                b"To the world you may be just one person, but to one person, you may be the world.",
                r"person"
            )
            .unwrap();
            assert_eq!(res.len(), 1);
            assert_eq!(res[0], (1, "person".to_string()));
        }

        /// Tests the `search_string` function with a pattern that has no matches.
        #[test]
        fn search_string_no_match() {
            let res = search_string(b"", r"not found").unwrap();
            assert_eq!(res.len(), 0);
            assert_eq!(res, vec![]);
        }
    }
}

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

    if let Some(short) = matches.get_one::<u8>("short") {
        get_quote(&short);
    } else {
        get_quote(&0);
    }

    Ok(())
}

fn random(i: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..i)
}

fn get_quote(quote_size: &u8) {
    let file = file::pick_file("fortunes".to_string()).unwrap();
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
            if *n == 255 as u8 {
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
            println!("{}", tmp[random(tmp.len())]);
        }
        _ => {
            println!("{}", quotes[random(quotes.len() - 1)]);
        }
    }
}

pub mod file {
    use rand::prelude::SliceRandom;
    use std::fs;
    use std::io::Read;

    /// Does not account for amount of fortunes
    pub fn pick_file(dir: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let files: Vec<_> = fs::read_dir(dir)?.collect();
        // println!("{:#?files}");
        let file = files.choose(&mut rng).ok_or("No files found")?;
        let path = file.as_ref().unwrap().path();

        let mut contents = String::new();
        fs::File::open(path)?.read_to_string(&mut contents)?;

        Ok(contents)
    }
}
