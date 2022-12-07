use std::ops::Index;

use crate::{BoardIndex, BoardIndexRangeInclusive, Direction, SquareContent};

/*
    Learnings in this module:

    * Working with tuples
    * Debug asserts
    * Private methods
    * Enum with values

    Recommended readings for this module:

    * Tuples: https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
    * `debug_assert`: https://doc.rust-lang.org/std/macro.debug_assert.html
    * Enum values: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values
*/

#[derive(Debug, PartialEq, Eq)]
pub enum ShipFindingResult {
    NoShip,
    CompleteShip(BoardIndexRangeInclusive),
    PartialShip(BoardIndexRangeInclusive),
}

pub trait ShipFinder {
    fn try_find_ship(&self, ix: BoardIndex) -> ShipFindingResult;
}

// Note: Private method returning tuple
fn find_ship_edge(
    board: &impl Index<BoardIndex, Output = SquareContent>,
    current: BoardIndex,
    direction: Direction,
    prev: bool,
) -> (BoardIndex, bool) {
    debug_assert!(board[current].is_ship());

    let mut current = current; // Note shadowing (immutable -> mutable)
    loop {
        let next = if prev {
            current.try_previous(direction)
        } else {
            current.try_next(direction)
        };

        // Note pattern matching with if let
        if let Some(x) = next {
            current = x;
        }

        if next == None || !board[current].is_ship() {
            break;
        }
    }

    if board[current].is_ship() {
        return (current, true);
    }

    let complete = board[current] != SquareContent::Unknown;
    match prev {
        true => (current.try_next(direction).unwrap(), complete),
        false => (current.try_previous(direction).unwrap(), complete),
    }
}

fn try_direction(
    board: &impl Index<BoardIndex, Output = SquareContent>,
    ix: BoardIndex,
    direction: Direction,
) -> (BoardIndexRangeInclusive, bool) {
    debug_assert!(board[ix].is_ship());

    let (beginning_ix, beginning_complete) = find_ship_edge(board, ix, direction, true);
    let (end_ix, end_complete) = find_ship_edge(board, ix, direction, false);
    (BoardIndexRangeInclusive::new(beginning_ix, end_ix), beginning_complete && end_complete)
}

impl<T> ShipFinder for T
where
    T: Index<BoardIndex, Output = SquareContent>,
{
    fn try_find_ship(&self, ix: BoardIndex) -> ShipFindingResult {
        if !self[ix].is_ship() {
            return ShipFindingResult::NoShip;
        }

        let mut result;
        for direction in [Direction::Horizontal, Direction::Vertical] {
            result = try_direction(self, ix, direction);
            if result.0.length() > 1 {
                return if result.1 {
                    ShipFindingResult::CompleteShip(result.0)
                } else {
                    ShipFindingResult::PartialShip(result.0)
                };
            }
        }

        // When only a single square of a ship is known, no orientation can be determined and therefore the ship is only partial.
        ShipFindingResult::PartialShip(BoardIndexRangeInclusive::new(ix, ix))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{BattleshipBoardContent, ShipSetter};

    use super::*;
    use rstest::rstest;

    #[test]
    fn find_ship_edge_horizontal_complete() {
        let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
        board.try_place_ship("A1".parse().unwrap(), 3, Direction::Horizontal).unwrap();

        let result =
            find_ship_edge(&board, BoardIndex::from_index(0), Direction::Horizontal, false);
        assert_eq!(BoardIndex::from_index(2), result.0);
        assert!(result.1);

        let result = find_ship_edge(&board, BoardIndex::from_index(2), Direction::Horizontal, true);
        assert_eq!(BoardIndex::from_index(0), result.0);
        assert!(result.1);
    }

    #[test]
    fn find_ship_edge_vertical_complete() {
        let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
        board.try_place_ship("A1".parse().unwrap(), 3, Direction::Vertical).unwrap();

        let result = find_ship_edge(&board, BoardIndex::from_index(0), Direction::Vertical, false);
        assert_eq!(BoardIndex::from_index(20), result.0);
        assert!(result.1);

        let result = find_ship_edge(&board, BoardIndex::from_index(20), Direction::Vertical, true);
        assert_eq!(BoardIndex::from_index(0), result.0);
        assert!(result.1);
    }

    #[test]
    fn find_ship_edge_horizontal_incomplete() {
        let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
        board.try_place_ship("B2".parse().unwrap(), 3, Direction::Horizontal).unwrap();
        board[BoardIndex::from_str("D2").unwrap()] = SquareContent::HitShip;
        board[BoardIndex::from_str("E2").unwrap()] = SquareContent::Unknown;
        board[BoardIndex::from_str("A2").unwrap()] = SquareContent::Unknown;

        let result = find_ship_edge(&board, "C2".parse().unwrap(), Direction::Horizontal, false);
        assert_eq!(result.0, "D2".parse().unwrap());
        assert!(!result.1);

        let result = find_ship_edge(&board, "C2".parse().unwrap(), Direction::Horizontal, true);
        assert_eq!(result.0, "B2".parse().unwrap());
        assert!(!result.1);
    }

    #[test]
    fn find_ship_edge_vertical_incomplete() {
        let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
        board.try_place_ship("B2".parse().unwrap(), 3, Direction::Vertical).unwrap();
        board[BoardIndex::from_str("B4").unwrap()] = SquareContent::HitShip;
        board[BoardIndex::from_str("B5").unwrap()] = SquareContent::Unknown;
        board[BoardIndex::from_str("B1").unwrap()] = SquareContent::Unknown;

        let result = find_ship_edge(&board, "B3".parse().unwrap(), Direction::Vertical, false);
        assert_eq!(result.0, "B4".parse().unwrap());
        assert!(!result.1);

        let result = find_ship_edge(&board, "B3".parse().unwrap(), Direction::Vertical, true);
        assert_eq!(result.0, "B2".parse().unwrap());
        assert!(!result.1);
    }

    #[test]
    fn find_ship_simple() {
        let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
        board.try_place_ship("A1".parse().unwrap(), 3, Direction::Horizontal).unwrap();

        assert_eq!(
            ShipFindingResult::CompleteShip(BoardIndexRangeInclusive::new(
                "A1".parse().unwrap(),
                "C1".parse().unwrap()
            )),
            board.try_find_ship("A1".parse().unwrap())
        );
    }

    #[test]
    fn find_ship_simple_partial() {
        let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
        board.try_place_ship("A1".parse().unwrap(), 3, Direction::Horizontal).unwrap();
        board[BoardIndex::from_str("D1").unwrap()] = SquareContent::Unknown;

        assert_eq!(
            ShipFindingResult::PartialShip(BoardIndexRangeInclusive::new(
                "A1".parse().unwrap(),
                "C1".parse().unwrap()
            )),
            board.try_find_ship("A1".parse().unwrap())
        );
    }

    #[test]
    fn find_ship_single_square() {
        let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
        board[BoardIndex::from_str("G10").unwrap()] = SquareContent::HitShip;
        board[BoardIndex::from_str("H10").unwrap()] = SquareContent::Unknown;

        assert_eq!(
            ShipFindingResult::PartialShip(BoardIndexRangeInclusive::new(
                "G10".parse().unwrap(),
                "G10".parse().unwrap()
            )),
            board.try_find_ship("G10".parse().unwrap())
        );
    }

    #[rstest]
    #[case(SquareContent::Water, true)]
    #[case(SquareContent::Unknown, false)]
    fn find_ship_horizontal(#[case] initial_content: SquareContent, #[case] complete: bool) {
        for col in 0..7 {
            for row in 0..10 {
                let mut board = BattleshipBoardContent::new_initialized(initial_content);
                let locations = &[
                    BoardIndex::from_col_row(col, row),
                    BoardIndex::from_col_row(col + 1, row),
                    BoardIndex::from_col_row(col + 2, row),
                ];
                locations.into_iter().for_each(|l| {
                    board[*l] = SquareContent::Ship;
                });
                locations.into_iter().for_each(|l| match board.try_find_ship(*l) {
                    ShipFindingResult::CompleteShip(x) if complete => assert_eq!(3, x.length()),
                    ShipFindingResult::PartialShip(x) if !complete => assert_eq!(3, x.length()),
                    _ => panic!("Invalid result"),
                });
            }
        }
    }

    #[rstest]
    #[case(SquareContent::Water, true)]
    #[case(SquareContent::Unknown, false)]
    fn find_ship_vertical(#[case] initial_content: SquareContent, #[case] complete: bool) {
        for col in 0..10 {
            for row in 0..7 {
                let mut board = BattleshipBoardContent::new_initialized(initial_content);
                let locations = &[
                    BoardIndex::from_col_row(col, row),
                    BoardIndex::from_col_row(col, row + 1),
                    BoardIndex::from_col_row(col, row + 2),
                ];
                locations.into_iter().for_each(|l| {
                    board[*l] = SquareContent::Ship;
                });
                locations.into_iter().for_each(|l| match board.try_find_ship(*l) {
                    ShipFindingResult::CompleteShip(x) if complete => assert_eq!(3, x.length()),
                    ShipFindingResult::PartialShip(x) if !complete => assert_eq!(3, x.length()),
                    _ => panic!("Invalid result"),
                });
            }
        }
    }
}
