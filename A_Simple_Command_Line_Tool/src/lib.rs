use std::error::Error;
use std::fs;    // For file handling
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// Creating a constractor for Config:
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Error Message by returning a Result instead of calling panic!:
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case, })
    }
}

/*
Annotation:
Here, for the error type, we used the trait object Box<dyn Error>. For now,
just know it means the function will return a type that implements the Error
trait, and don't have to specify what type the return value will be.
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {  // dyn == dynamic
    // Reading a File:
    // open the file & returns a value of type std::io::Result<String>
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    if results.is_empty() {
        println!("Not found!!!");
    } else {
        println!("Found:");
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
