#![allow(dead_code)]

const SIZE: usize = 3;

fn main() {
    let mut board = [[' '; SIZE]; SIZE];
    let mut current_player = 'X';

    loop {
        print_board(&board);

        let location = read_location();
        println!("{location:?}");

        board[location.row][location.col] = current_player;
        let winner = get_winner(&board);
        if winner != ' ' {
            print_board(&board);
            println!("{} wins!", winner);
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

/// Get the winner in the tic tac toe game; return blank if no winner
fn get_winner(board: &[[char; SIZE]; SIZE]) -> char {
    for row in 0..3 {
        if board[row][0] != ' ' && board[row][0] == board[row][1] && board[row][1] == board[row][2] {
            return board[row][0];
        }
    }

    for col in 0..3 {
        if board[0][col] != ' ' && board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            return board[0][col];
        }
    }

    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return board[0][0];
    }

    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return board[0][2];
    }

    ' '
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

fn print_board(board: &[[char; SIZE];SIZE]) {
    println!("┌───┬───┬───┐");
    for row in 0..SIZE {
        print!("│");
        for col in 0..SIZE {
            print!(" {} │", board[row][col]);
        }
        println!();
        if row < SIZE - 1 {
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
