use clap::{Arg, Command};
use std::fs;

fn main() {
    let matches = Command::new("minicat")
        .version("0.1")
        .about("A mini version of cat")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("File to print")
            .required(true))
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap();
    match fs::read_to_string(file) {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

// The idea from Rust Cookbook Argument Parsing,
// visit "https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html"