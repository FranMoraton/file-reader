use std::{fs, error::Error};

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

pub fn run(config: Config)-> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    println!("{}", contents);
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
