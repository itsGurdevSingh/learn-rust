use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_senstive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_senstive = env::var("CASE_INSENSITIVE").is_err();

        println!("case senstive value form env{:?}", env::var("CASE_INSENSITIVE"));
        return Ok(Config { query, filename , case_senstive});
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("searching file for {}", config.query);
    println!("file {} :-\n", config.filename);

    let content = fs::read_to_string(config.filename)?;

    let results: Vec<&str> = if config.case_senstive {
        search_case_senstive(&config.query, &content)
    }else {
        search_case_insenstive(&config.query, &content)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_senstive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insenstive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_senstive() {
        let query = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Duct tape.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_senstive(query, contents));
    }

    #[test]
    fn case_insenstive(){
        let query: &str = "rUsT";
        let contents: &str = "\
Rust
safe, fast, prodctive.
Dust tape.
trust me";

        assert_eq!(vec!["Rust", "trust me"], search_case_insenstive(query, contents));

    }
}
