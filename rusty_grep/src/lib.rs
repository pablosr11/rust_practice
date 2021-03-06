use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enuff args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let sensitive = env::var("RUSTY_SENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", &line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(&line.trim())
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(&line.trim())
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let q = "rust";
        let c = "\
        Honey is sweet and suit.
        But it is not.
        The crust -
        ";

        assert_eq!(vec!["The crust -"], search(q, c));
    }

    #[test]
    fn case_insensitive() {
        let q = "CrUsT";
        let c = "\
        Honey is sweet and suit.
        But it is not.
        The crust -
        ";

        assert_eq!(vec!["The crust -"], search_case_insensitive(q, c))
    }
}
