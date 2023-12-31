use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}


impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path string")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config {
            query, 
            file_path,
            ignore_case
         })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        let search_results = search(& config.query, & contents);
        println!("The result are: {:?}", search_results);
        Ok(())
    }
}


pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn case_sensitive_search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
