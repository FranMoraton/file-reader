use std::{error::Error, fs, env};

pub struct Config {
    haystack: String,
    file: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let haystack = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a haystack string"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        
        let case_sensitive= match env::var("CASE_SENSITIVE") {
            Ok(n) => n.parse().unwrap(),
            Err(_) => false,
        };

        Ok(Config {
            haystack,
            file,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    let results = if config.case_sensitive {
        search(&config.haystack, &contents)
    } else {
        search_insensitive(&config.haystack, &contents)
    };

    for line in results {
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
