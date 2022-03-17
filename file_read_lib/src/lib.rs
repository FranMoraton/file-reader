use std::{error::Error, fs};

pub struct Config {
    haystack: String,
    file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough parameters expect haystack and file");
        }

        Ok(Config {
            haystack: args[1].to_owned(),
            file: args[2].to_owned(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    for line in search(&config.haystack, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(haystack: &str, contents: &'a str)-> Vec<&'a str> {
    let mut lines_containing_haystack = Vec::new();

    for line in contents.lines() {
        if line.contains(haystack) {
            lines_containing_haystack.push(line);
        }
    }

    lines_containing_haystack
}

pub fn search_insensitive<'a>(haystack: &str, contents: &'a str)-> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&haystack.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_insensitive(query, contents));
    }
}
