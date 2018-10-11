use std::error::Error;
use std::fs;

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

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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
        ]).expect("config should be valid");
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn new_invalid_config() {
        Config::new(&vec![String::from("minigrep"), String::from("foo")])
            .expect("should panic, not enough args");
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn run_no_file() {
        let config = Config::new(&vec![
            String::from("minigrep"),
            String::from("the"),
            String::from("nope"),
        ]).unwrap();
        run(config).expect("file was found when it shouldn't be");
    }

    #[test]
    fn run_ok_when_file_exists() {
        let config = Config::new(&vec![
            String::from("minigrep"),
            String::from("the"),
            String::from("poem.txt"),
        ]).unwrap();
        run(config).expect("couldn't run with config")
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
