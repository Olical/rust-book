use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for result in results {
        println!("{}", result);
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

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
