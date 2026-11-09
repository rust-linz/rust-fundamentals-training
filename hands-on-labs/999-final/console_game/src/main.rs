use battleship_game_logic::{BattleshipBoardContent, BoardFiller, SquareContent, random_placer};
use clap::{Parser, Subcommand};

/*
    Learnings in this module:

    * Build a CLI with Rust

    Recommended readings for this module:

    * clap derive tutorial: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
*/

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    SquareContent,
    Board {
        #[arg(short, long, help = "Indicates whether the board should be filled")]
        fill: bool
    },
}

fn main() {
    match Cli::parse().command {
        Command::SquareContent => square_content(),
        Command::Board { fill } => board(fill),
    }
}

fn square_content() {
    let content = SquareContent::default();
    println!("Debug: {:?}", content);
    
    let content: SquareContent = '~'.into();
    println!("Parsed: {:?}", content);
    println!("Parsed Display: {}", char::from(content));
}

fn board(fill: bool) {
    let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
    if fill { board.fill(&[5, 4, 3, 3, 2], random_placer); }

    println!("Filled board:\n{}", board);
}
