use std::env;
use std::process;
use cli_file_search::Config;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    }); 
    
    if let Err(e) = Config::run(config) {
        eprintln!("Problem searching through the file: {e}")
    }
}

