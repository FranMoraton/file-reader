use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    run(config);
}

struct Config {
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

fn run(config: Config) {
    let contents = fs::read_to_string(config.file).expect("file not valid");
    println!("{}", contents);
}
