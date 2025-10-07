use ansi_term::Colour;
use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    println!("Hello!");
    println!("You can enter a something now, for exit, enter exit!");

    let mut flag = false;

    loop {
        let mut phrase = String::new();
        let mut color = String::new();

        println!("Enter something:");
        io::stdout().flush()?;
        io::stdin().read_line(&mut phrase)?;
        let phrase = phrase.trim();

        if phrase.is_empty() {
            println!("No valid color entered! Please try again:");
            continue;
        } else if phrase == "exit" {
            println!("Exiting...");
            flag = true;
            break;
        } else {
            break;
        }
    }

    if !flag {
        loop {
            println!("Enter the color in lower-case:");
            io::stdout().flush()?;
            io::stdin().read_line(&mut color);
            let color = color.trim();

            if color.is_empty() {
                println!("No valid color entered! Please try again:");
                continue;
            }
        }

        let output = match color {
            "red" => Colour::Red.paint(&phrase),
            "green" => Colour::Green.paint(&phrase),
            "yellow" => Colour::Yellow.paint(&phrase),
            "blue" => Colour::Blue.paint(&phrase),
            "black" => Colour::Black.paint(&phrase),
            "purple" => Colour::Purple.paint(&phrase),
            "cyan" => Colour::Cyan.paint(&phrase),
            "white" => Colour::White.paint(&phrase),
            _ => println!(
                "Sorry this color is not available"
            )
        };

        println!("Your output:");
        println!("{}", output);
    }

    Ok(())
}
