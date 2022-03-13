use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file).expect("file not valid");
    println!("{}", contents);
}


struct Config {
    haystack: String,
    file: String,
}

impl Config {
    pub fn new(args: &[String])-> Config {
        Config {
            haystack: args[1].to_owned(),
            file: args[2].to_owned(),
        }
    }
}