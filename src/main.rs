use std::io;

mod cli;
mod file;
mod fortune;
mod random;

fn main() -> io::Result<()> {
    let matches = cli::build_cli().get_matches();

    if let Some(pattern) = matches.get_one::<String>("find") {
        fortune::search_fortunes(pattern);
    } else if let Some(short) = matches.get_one::<u8>("short") {
        fortune::get_quote(short);
    } else {
        fortune::get_quote(&0);
    }

    Ok(())
}
