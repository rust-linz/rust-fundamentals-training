use std::{env::args, process::ExitCode};

mod args;
mod ascii_art;
mod pyramid;

use crate::{
    args::get_base_length,
    ascii_art::{
        Decorator, INDENTATION, repeated_decorator, repeated_decorator_iter, repeated_numbers_iter,
    },
    pyramid::calculate_pyramid,
};

// ================================================================================
// LEARNINGS:
// - Returning an exit code from main
// - Match expressions
// - Working with iterators
// - Slices and vectors
// - Type inference
// - Closures
// ================================================================================

fn main() -> ExitCode {
    // Get the base length of the math pyramid from the command line arguments.
    let base_length = match get_base_length(args()) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Usage error: {e}");
            return ExitCode::FAILURE; // Indicate error by returning a non-zero exit code.
        }
    };

    // Generate random numbers used as the base of the math pyramid.
    let random_base_numbers: Vec<u16> = (0..base_length)
        .map(|_| rand::random_range(0..10))
        .collect();

    // Allocate a single string containing the max indentation of the math pyramid.
    let max_indentation = INDENTATION.repeat(base_length - 1);

    // Trigger the recursive generation of the math pyramid.
    calculate_pyramid(&random_base_numbers, 0, &|level, numbers| {
        // Calculate the indentation of the current level. Note that we are not allocating
        // memory here. We are just using a slice of the max indentation string.
        let indentation = &max_indentation[0..level * INDENTATION.len()];

        // Draw top border using `repeated_decorator`
        print!("{}", indentation);
        println!("{}", repeated_decorator(Decorator::Top, numbers.len()));

        // Draw the numbers using `repeated_numbers_iter`
        print!("{}", indentation);
        println!("{}", repeated_numbers_iter(numbers.iter().copied()));

        // Draw bottom border using `repeated_decorator_iter`
        print!("{}", indentation);
        repeated_decorator_iter(Decorator::Bottom, numbers.len()).for_each(|s| print!("{s}"));
        println!();
    });

    ExitCode::SUCCESS
}
