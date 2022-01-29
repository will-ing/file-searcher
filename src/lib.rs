use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file and store it as a String. Panic if something goes wrong.
    let contents = fs::read_to_string(config.file)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file"),
        };

        // To use in bash enter: export CASE_SENSITIVE=true / unset CASE_SENSITIVE
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query,
            file,
            case_sensitive,
        })
    }
}

// return needs a specified lifetime. Assign to contents parameter.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    /* let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results*/
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Trust Me";
        assert_eq!(
            vec!["Rust:", "Trust Me"],
            search_case_insensitive(query, contents)
        )
    }
}
