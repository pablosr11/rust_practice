use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() == 3 {
        false => Err("Missing filename or query"),
        true => Ok("yei"),
    }
    .expect("missing arg");

    let query = &args[1];
    let filename = &args[2];

    println!("Finding {} in {}", &query, &filename);

    let contents = fs::read_to_string(&filename).expect("Couldnt read the file");

    println!("{}", &contents);
}
