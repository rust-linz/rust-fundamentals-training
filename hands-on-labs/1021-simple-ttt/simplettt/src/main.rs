use board_content::BoardContent;
use board_index::BoardIndex;

mod board_content;
mod board_index;
mod game;

fn main() {
    let mut game = game::Game::new(BoardContent::new());

    println!("{}", game.content);

    while game.get_winner().is_none() && !game.content.is_full() {
        let loc = read_location();
        if let Err(e) = game.set(loc) {
            println!("{e}");
            continue;
        }

        println!("{}", game.content);
    }

    match game.get_winner() {
        Some(winner) => println!("Winner: {}", char::from(winner)),
        None => println!("It's a draw!"),
    }
}

/// Reads location from the user through stdin/stdout
fn read_location() -> BoardIndex {
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

        // Parse the input using our FromStr implementation. Note that `trim`
        // removes the trailing line break (\n on Linux/macOS, \r\n on Windows).
        match input.trim().parse() {
            Ok(location) => break location,
            Err(e) => println!("Invalid location: {e}"), // Ask again
        }
    }
}
