use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Issue found parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Finding {} in {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("Couldnt read the file");

    println!("{}", &contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enuff args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
