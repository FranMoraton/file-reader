use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file = &args[2];

    println!("{:?}", query);
    println!("{:?}", file);

    let contents = fs::read_to_string(file).expect("file not valid");
    println!("{}", contents);
}
