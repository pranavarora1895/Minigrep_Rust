//! # MiniGrep ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
//!
//! ### This is a Command line utility developed in Rust that parses the keyword from the file and outputs the relevant line.
//!
//! ### Clone this repo and follow the given usage:
//!
//! > ## Usage
//! >
//! > * `cargo run <keyword> <filename.txt>`
//! > * Setup the `CASE_INSENSITIVE` environment variable
//! > * On your PowerShell run the following commands:
//! >```
//! > $Env CASE_INSENSITIVE=1
//! > cargo run <keyword> <filename.txt>
//! >```
//! > * To deactivate the Environment Variable, run the following command:
//! >  
//! > `Remove-Item Env:CASE_INSENSITIVE`
//! ---
//!
//! * Follow Me On Instagram at [Pranav Arora](https://www.instagram.com/arorapranav187)
//! * Lets Get Connected on Linkedin at [Pranav Arora](https://www.linkedin.com/in/pranav-arora-354b71bb/)
//!
//! ### ThankYou!

use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// Writing Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

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
