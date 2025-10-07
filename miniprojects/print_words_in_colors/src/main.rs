use ansi_term::Colour;
use ansi_term::Style;
use std::io;

fn main() {
    println!("Hello!");
    println!("You can enter a phrase now, if you need help, enter help!");

    let mut phrase = String::new();

}

fn read_phrase(
    phrase: &mut String, color: &mut String
) -> Result<(), std::io::Error> {
    io::stdin()
        .read_line(&mut phrase)
        .expect("Failed to read line for some reasons...")?;

    let phrase = phrase.trim();

    io::stdin()
        .read_line(&mut color)
        .expect("Did not get any color instruction, try again")?;

    println!("The stuff you entered: {}", phrase);
}