use rand::prelude::*;
use std::env;
use std::io;

// 1. Create a random number
// 2. Read from stdin
// 3. Parse CLI arguments

fn main() {
    let mut rng = rand::rng();
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let number_of_guesses: u32 = args[1].parse().expect("Could not parse number of tries");
        let number_to_guess: u32 =
            rng.random_range(0..=args[2].parse().expect("Could not parse number to guess"));

        for i in 1..=number_of_guesses {
            let mut buffer = String::new();
            let _read_bytes = io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from CL");
            let guessed_number: u32 = buffer.trim().parse().expect("Could not parse number");

            let msg = match guessed_number.cmp(&number_to_guess) {
                std::cmp::Ordering::Less => "Too low",
                std::cmp::Ordering::Greater => "Too high",
                std::cmp::Ordering::Equal => {
                    println!("Just about right after {} steps", i);
                    break;
                }
            };

            println!("{}", msg);
        }
    }
}
