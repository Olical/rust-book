use std::error::Error;
use std::fs;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid_config() {
        assert!(
            Config::new(&vec![
                String::from("minigrep"),
                String::from("foo"),
                String::from("bar")
            ]).is_ok(),
            "config is not okay"
        );
    }

    #[test]
    fn new_invalid_config() {
        assert!(
            Config::new(&vec![String::from("minigrep"), String::from("foo")]).is_err(),
            "config is not invalid"
        );
    }
}
