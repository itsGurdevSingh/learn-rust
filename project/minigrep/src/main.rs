use std::env;
use std::error::Error;
use std::fs;
use std::process;


struct Config {
    query : String,
    filename : String
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config =  Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config){
        println!("Application Error : {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("searching file for {}", config.query);
    println!("file {} :-\n", config.filename);

    let content = fs::read_to_string(config.filename)?;

    println!("{}",content);

    Ok(())
}
