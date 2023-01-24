use board_index::BoardIndex;

use crate::{board_content::{BoardContent, SquareContent}, game::WinnerDetector};

mod board_index;
mod board_content;
mod game;

fn main() {
    let mut game = game::Game::new(BoardContent::new());

    println!("{}", game.content);
    
    while game.get_winner().is_none() {
        let loc = read_location();
        game.set(loc).unwrap();
        println!("{}", game.content);
    }

    println!("Winner: {:?}", match game.get_winner() {
        Some(SquareContent::X) => "X",
        Some(SquareContent::O) => "O",
        None => "None",
    });
}


/// Reads location from the user through stdin/stdout
fn read_location() -> BoardIndex {
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
                break BoardIndex::from_col_row(col as usize, row as usize);
            }
        }

        // Input is invalid, ask again
        println!("Invalid location");
    }
}