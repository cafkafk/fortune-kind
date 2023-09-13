use clap::{arg, command, crate_authors, Arg};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::process::exit;

const SHORT: usize = 150;

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
    let file = pick_file("fortunes".to_string()).unwrap();
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

/// Does not account for amount of fortunes
fn pick_file(dir: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let files: Vec<_> = fs::read_dir(dir)?.collect();
    // println!("{:#?files}");
    let file = files.choose(&mut rng).ok_or("No files found")?;
    let path = file.as_ref().unwrap().path();

    let mut contents = String::new();
    fs::File::open(path)?.read_to_string(&mut contents)?;

    Ok(contents)
}
