use std::{env, process};

use file_read_lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = file_read_lib::Config::new(&args).unwrap_or_else(|err| {
        println!("application error: {}", err);
        process::exit(1);
    });

    if let Err(err) = file_read_lib::run(config) {
        println!("application error: {}", err);
        process::exit(1);
    };
}
