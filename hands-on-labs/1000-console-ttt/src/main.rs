#![allow(dead_code)]

fn main() {
    // Print sample boards
    print_board();
    print_colored_board();

    // Sample game loop
    loop {
        let location = read_location();
        println!("{location:?}");
    }
}

/// Location on a Tic Tac Toe game
#[derive(Debug)]
struct Location {
    /// Zero-based row (0..2)
    row: usize,

    /// Zero-based column (0..2)
    col: usize,
}

/// Reads location from the user through stdin/stdout
fn read_location() -> Location {
    use std::io::{stdin, stdout, Write};

    loop {
        // Ask the user for a location
        print!("Enter location (A1-C3): ");
        stdout().flush().unwrap(); // Need to flush to ensure text is really written on the screen

        // Read the input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.as_bytes();

        // Pare the input. Input must be in the form of A1, B2, etc. plus trailing \n
        if input.len() == 3 {
            let row = input[0] - b'A';
            let col = input[1] - b'1';
            if row < 3 && col < 3 {
                // Input is fine, end loop with the result
                break Location {
                    row: row as usize,
                    col: col as usize,
                };
            }
        }

        // Input is invalid, ask again
        println!("Invalid location");
    }
}

/// Sample for printing a nice board
fn print_board() {
    println!(
        r"    
┌───┬───┬───┐
│ X │ O │ O │
├───┼───┼───┤
│   │ X │   │
├───┼───┼───┤
│   │   │   │
└───┴───┴───┘"
    );
}

/// Sample for printing a nice board with colors (e.g. to highlight winning combinations)
fn print_colored_board() {
    use colored::*;

    println!(
        r"    
┌───┬───┬───┐
│{}│ O │ O │
├───┼───┼───┤
│   │ X │   │
├───┼───┼───┤
│   │   │   │
└───┴───┴───┘",
        " X ".on_white().red()
    );
}
