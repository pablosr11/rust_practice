use rusty_grep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Issue found parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rusty_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
