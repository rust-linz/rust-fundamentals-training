#![allow(dead_code)]

const SIZE: usize = 3;

fn main() {
    let mut board = [[' '; SIZE]; SIZE];
    let mut current_player = 'X';

    loop {
        print_board(&board);

        let location = read_location();
        println!("{location:?}");

        // A square can only be set once, ask again if it is already taken
        if board[location.row][location.col] != ' ' {
            println!("Square is already taken");
            continue;
        }

        board[location.row][location.col] = current_player;
        let winner = get_winner(&board);
        if winner != ' ' {
            print_board(&board);
            println!("{winner} wins!");
            break;
        }

        // No winner and no empty square left: The game ends in a draw
        if board.iter().flatten().all(|&square| square != ' ') {
            print_board(&board);
            println!("It's a draw!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
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

/// All lines that win the game (rows, columns, diagonals) as (row, col) coordinates
const WINNING_LINES: [[(usize, usize); SIZE]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

/// Get the winner in the tic tac toe game; return blank if no winner
fn get_winner(board: &[[char; SIZE]; SIZE]) -> char {
    for line in WINNING_LINES {
        let [a, b, c] = line.map(|(row, col)| board[row][col]);
        if a != ' ' && a == b && b == c {
            return a;
        }
    }

    ' '
}

/// Reads location from the user through stdin/stdout
fn read_location() -> Location {
    use std::io::{Write, stdin, stdout};

    loop {
        // Ask the user for a location
        print!("Enter location (A1-C3): ");
        stdout().flush().unwrap(); // Need to flush to ensure text is really written on the screen

        // Read the input
        let mut input = String::new();
        if stdin().read_line(&mut input).unwrap() == 0 {
            // End of input (e.g. Ctrl+D), asking again would loop forever
            eprintln!("No more input, ending game");
            std::process::exit(1);
        }

        // Parse the input. Input must be in the form of A1, B2, etc. (letter = column,
        // digit = row). Note that `trim` removes the trailing line break
        // (\n on Linux/macOS, \r\n on Windows).
        if let [col @ b'A'..=b'C', row @ b'1'..=b'3'] = input.trim().as_bytes() {
            // Input is fine, end loop with the result
            break Location {
                row: (row - b'1') as usize,
                col: (col - b'A') as usize,
            };
        }

        // Input is invalid, ask again
        println!("Invalid location");
    }
}

fn print_board(board: &[[char; SIZE]; SIZE]) {
    println!("┌───┬───┬───┐");
    for (row_ix, row) in board.iter().enumerate() {
        print!("│");
        for item in row {
            print!(" {item} │");
        }
        println!();
        if row_ix < SIZE - 1 {
            println!("├───┼───┼───┤");
        }
    }
    println!("└───┴───┴───┘");
}

/// Sample for printing a nice board
fn print_board_sample() {
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
