use ansi_term::Colour;
use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    println!("Hello!");
    println!("You can enter a something now, for exit, enter exit!");

    let mut flag = false;
    let mut phrase = String::new();
    let mut better_phrase = String::new();
    let mut color = String::new();

    loop {
        println!("Enter something:");
        io::stdout().flush()?;
        io::stdin().read_line(&mut phrase)?;
        better_phrase = phrase.trim().to_string();

        if better_phrase.is_empty() {
            println!("No valid color entered! Please try again:");
            continue;
        } else if better_phrase == "exit" {
            println!("Exiting...");
            flag = true;
            break;
        } else {
            break;
        }
    }

    if !flag {
        let mut better_color;
        loop {
            println!("Enter the color in lower-case:");
            io::stdout().flush()?;
            io::stdin().read_line(&mut color)?;
            better_color = color.trim();

            if better_color.is_empty() {
                println!("No valid color entered! Please try again:");
                continue;
            } else {
                break;
            }
        }

        let output = match better_color {
            "red" => Colour::Red.paint(&better_phrase),
            "green" => Colour::Green.paint(&better_phrase),
            "yellow" => Colour::Yellow.paint(&better_phrase),
            "blue" => Colour::Blue.paint(&better_phrase),
            "black" => Colour::Black.paint(&better_phrase),
            "purple" => Colour::Purple.paint(&better_phrase),
            "cyan" => Colour::Cyan.paint(&better_phrase),
            "white" => Colour::White.paint(&better_phrase),
            _ => Colour::White.paint(
                "Sorry this color is not available"
            ),
        };

        println!("Your output:");
        println!("{}", output);
    }

    Ok(())
}
