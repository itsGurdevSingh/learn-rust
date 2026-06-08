use std::error::Error;
use std::fs;


pub struct Config {
   pub query : String,
   pub filename : String
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("searching file for {}", config.query);
    println!("file {} :-\n", config.filename);

    let content = fs::read_to_string(config.filename)?;

    println!("{}",content);

    Ok(())
}