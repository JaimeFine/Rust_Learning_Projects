use ansi_term::Colour;
use std::io::{self, Write};

fn main() {
    println!("Hello!");
    println!("You can enter a something now, if you need help, enter help!");

    loop {
        let mut phrase = String::new();
        let mut color = String::new();

        println!("Enter the something:");
        io::stdout.flush()?;
        io::stdin().read_line(&mut phrase)?;
        let phrase = phrase.trim();

        if phrase.is_empty() {
            println!("No valid color entered! Please try again:");
            continue;
        }

        println!("Enter the color:");
        io::stdout.flush()?;
        io::stdin().read_line(&mut color);
        let color = color.trim();

        if color.is_empty() {
            println!("No valid color entered! Please try again:");
            continue;
        }
    }
}
