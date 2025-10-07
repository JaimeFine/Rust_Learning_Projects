use ansi_term::Colour;
use ansi_term::Style;
use std::io;

fn main() {
    println!("Hello!");
    println!("You can enter a phrase now, if you need help, enter help!");

    let mut phrase = String::new();

    io::stdin()
        .read_line(&mut phrase)
        .expect("Failed to read line for some reasons...");

    let phrase = phrase.trim();

    println!("The stuff you entered: {}", phrase);
}