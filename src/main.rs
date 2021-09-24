use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");

    let random_number = rand::thread_rng().gen_range(0..101);
    loop {
        println!("Input noooomber");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read ln");

        // trim the newline added by pressing enter
        // parse returns a result enum, use it to match
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("TOO SMOL"),
            Ordering::Greater => println!("TOO BANANA?"),
            Ordering::Equal => {
                println!("U WON");
                break;
            }
        }
    }
}
