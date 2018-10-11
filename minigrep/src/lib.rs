use std::error::Error;
use std::fs;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
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
        Config::new(&vec![
            String::from("minigrep"),
            String::from("foo"),
            String::from("bar"),
        ]).unwrap();
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn new_invalid_config() {
        Config::new(&vec![String::from("minigrep"), String::from("foo")]).unwrap();
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn run_no_file() {
        let config = Config::new(&vec![
            String::from("minigrep"),
            String::from("the"),
            String::from("nope"),
        ]).unwrap();
        run(config).unwrap();
    }

    #[test]
    fn run_ok_when_file_exists() {
        let config = Config::new(&vec![
            String::from("minigrep"),
            String::from("the"),
            String::from("poem.txt"),
        ]).unwrap();
        run(config).unwrap()
    }
}
