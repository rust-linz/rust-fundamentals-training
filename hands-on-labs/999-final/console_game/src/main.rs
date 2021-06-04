use battleship_game_logic::*;

fn main() {
    let content = SquareContent::Water;
    println!("{:?}", content);

    let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
    board[0] = SquareContent::Ship;
    board[1] = SquareContent::HitShip;

    println!("{}", board);
}
