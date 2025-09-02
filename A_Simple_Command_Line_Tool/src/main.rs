use std::env;   // Interacting with environment
use std::process;   // For closure: calling process::exit...

use A_Simple_Command_Line_Tool::Config;   // Using the lib.rs crate

fn main() {
    // Reading the Argument Values:
    let args: Vec<String> = env::args().collect();
    // Mind that: args[0] is "~/target/debug/minigrep.exe" <- .exe on Windows.

    // Saving the Argument Values &
    // Calling Config::build and Handling Errors:
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    /*
    Annotation:
    Using unwrap_or_else allows us to define some custom, non-panic! error
    handling. If the Result is an Ok value, this method's behavior is fimilar
    to unwrap: returning the inner value that Ok is wrapping. If it is a Err
    value, this method calls the code in the closure...
    */

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Handling Errors Returned from run in main:
    // Here we use if let rather than unwrap_or_else:
    if let Err(e) = A_Simple_Command_Line_Tool::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
