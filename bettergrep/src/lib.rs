use std::error::Error;
use std::fs;
use regex::Regex;

pub enum SearchMethod {
    Normal,
    CaseInsensitiveNormal,
    Strict,
    CaseInsensitiveStrict,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub search_method: SearchMethod,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let mut ignore_case = false;
        let mut strict = false;

        if let Some(arg) = args.next() {
            match arg.as_str() {
                "--strict" => strict = true,
                "--ignore-case" => ignore_case = true,
                _ => return Err("Unexpected argument given"),
            }
        }

        let search_method = match (strict, ignore_case) {
            (true, true) => SearchMethod::CaseInsensitiveStrict,
            (true, false) => SearchMethod::Strict,
            (false, true) => SearchMethod::CaseInsensitiveNormal,
            (false, false) => SearchMethod::Normal,
        };

        Ok(Config {
            query,
            file_path,
            search_method,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = match config.search_method {
        SearchMethod::Normal => {
            search(&contents, &config.query)
        },
        SearchMethod::Strict => {
            search_strict(&contents, &config.query)
        },
        SearchMethod::CaseInsensitiveNormal => {
            search_case_insensitive(&contents, &config.query)
        },
        SearchMethod::CaseInsensitiveStrict => {
            search_case_insensitive_strict(&contents, &config.query)
        },
    };

    if results.is_empty() {
        println!("Not found!!!");
    } else {
        println!("Found:");
        for (line_number, line) in results {
            println!("{line_number}: {line}");
        }
    }

    Ok(())
}

fn search_generic<'a, F>(query: &str, contents: &'a str, filter_func: F) -> Vec<(usize, &'a str)>
where
    // Returning a bool because filter_func tells us whether to keep or discard the current item:
    F: Fn(&str) -> bool,
{
    let words: Vec<&str> = contents.split_whitespace().collect();
    let words_number: u64 = words
        .iter()
        .filter(|&word| filter_func(word))
        .count() as u64;
    println!("There is a total of {} '{}' in the file:", words_number, query);

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| filter_func(line))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    search_generic(query, contents, |text| text.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query_lower = query.to_lowercase();
    search_generic(query, contents, |text| text.to_lowercase().contains(&query_lower))
}

pub fn search_strict<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let re = Regex::new(&format!(r"\b{}\b", regex::escape(query))).unwrap();
    search_generic(query, contents, |text| re.is_match(text))
}

pub fn search_case_insensitive_strict<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query_lower = query.to_lowercase();
    let re = Regex::new(&format!(r"\b{}\b", regex::escape(&query_lower))).unwrap();
    search_generic(query, contents, |text| re.is_match(&text.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_search_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec![(2, "safe, fast, productive.")], search(query, contents));
    }

    #[test]
    fn case_insensitive_search_test() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn search_strict_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three, no duct.
Hell yes!";
        assert_eq!(
            vec![(3, "Pick three, no duct.")],
            search_strict(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive_strict_test() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three, no rust.
Trust me!";
        assert_eq!(
            vec![(1, "Rust:"), (3, "Pick three, no rust.")],
            search_case_insensitive_strict(query, contents)
        );
    }
}