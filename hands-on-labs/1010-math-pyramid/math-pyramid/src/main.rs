use std::env::args;

mod args;

use crate::{
    args::get_base_length,
    ascii_art::{repeated_decorator, repeated_decorator_iter, repeated_numbers_iter, Decorator},
};

mod ascii_art;

fn main() {
    // Get the base length of the math pyramid from the command line arguments.
    let base_length = get_base_length(args());

    // Print error if base length is invalid.
    if base_length.is_err() {
        println!("Usage error: {}", base_length.unwrap_err());
        return;
    }

    // Note shadowing here
    let base_length = base_length.unwrap();

    let random_base_numbers: Vec<usize> = (0..base_length)
        .map(|_| rand::random::<usize>() % 10)
        .collect();

    // Draw top border using `repeated_decorator`
    println!("{}", repeated_decorator(Decorator::Top, base_length));

    println!("{}", repeated_numbers_iter(random_base_numbers.into_iter()));

    // Draw top border using `repeated_decorator_iter`
    repeated_decorator_iter(Decorator::Bottom, base_length).for_each(|s| print!("{s}"));
    println!();
}
