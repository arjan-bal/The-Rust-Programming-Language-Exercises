use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a pattern string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            pattern,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search(&config.pattern, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(pattern: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

fn search_case_insensitive<'a>(pattern: &'a str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Dict tape.";
        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(pattern, contents)
        );
    }
}
