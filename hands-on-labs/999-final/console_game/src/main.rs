use structopt::StructOpt;
use battleship_game_logic::*;

/*
    Learnings in this module:

    * Build a CLI with Rust

    Recommended readings for this module:

    * structopt crate: https://docs.rs/structopt/0.3.22/structopt/index.html
*/

#[derive(StructOpt)]
#[structopt()]
enum Command {
    Index,
    Board {
        #[structopt(short, long, help = "Indicates whether the board should be filled")]
        fill: bool
    },
}

fn main() {
    match Command::from_args() {
        Command::Index => index(),
        Command::Board { fill } => board(fill),
    }
}

fn index() {
    let ix = BoardIndex::new();
    println!("Debug: {:?}", ix);
    println!("Display: {}", ix);

    let ix = BoardIndex::from("B2");
    println!("Parsed: {}", ix);
}

fn board(fill: bool) {
    let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
    if fill { board.fill(&[5, 4, 3, 3, 2], random_placer); }

    println!("Filled board:\n{}", board);
}
