use std::{env, process};

use file_read_lib::{Config};

fn main() {
    let args: std::env::Args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("application error: {}", err);
        process::exit(1);
    });

    if let Err(err) = file_read_lib::run(config) {
        eprintln!("application error: {}", err);
        process::exit(1);
    };
}
