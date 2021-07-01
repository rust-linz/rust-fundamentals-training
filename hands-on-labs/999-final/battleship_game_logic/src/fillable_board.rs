use std::ops::Index;

use mockall::automock;

use crate::{BoardIndex, SquareContent, BOARD_SIDE_LENGTH};

/*
    Learnings in this module:

    * Implement a trait based on another trait
    * Custom error type
    * Local functions
    * Mock objects for unit testingt
    * Trait objects

    Recommended readings for this module:

    * mockall crate: https://github.com/asomers/mockall
    * Defining error types: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
    * Trait objects: https://doc.rust-lang.org/reference/types/trait-object.html
*/

pub enum Direction {
    Horizontal,
    Vertical,
}

#[automock]
pub trait PlacementCheck {
    fn can_place_ship(&self, ix: BoardIndex) -> bool;
}

impl PlacementCheck for dyn Index<BoardIndex, Output = SquareContent> {
    fn can_place_ship(&self, ix: BoardIndex) -> bool {
        self[ix] == SquareContent::Water
    }
}

pub trait FillableBoard {
    fn try_place_ship(
        &self,
        ix: BoardIndex,
        ship_length: usize,
        direction: Direction,
    ) -> Result<bool, PlacementError>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PlacementError {
    error_kind: ErrorKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    ShipTooLong,
    OutOfBounds,
    BoardTooOccupied,
}

impl FillableBoard for dyn PlacementCheck {
    fn try_place_ship(
        &self,
        ix: BoardIndex,
        ship_length: usize,
        direction: Direction,
    ) -> Result<bool, PlacementError> {
        if ship_length > BOARD_SIDE_LENGTH {
            return Err(PlacementError { error_kind: ErrorKind::ShipTooLong });
        }

        fn outside_bounds(start: usize, ship_length: usize) -> bool {
            start + ship_length > 10
        }
        if outside_bounds(
            match direction {
                Direction::Vertical => ix.row(),
                Direction::Horizontal => ix.column(),
            },
            ship_length,
        ) {
            return Err(PlacementError { error_kind: ErrorKind::OutOfBounds });
        }

        fn get_first(ix: usize) -> usize {
            // Note: if expression (not statement)
            if ix == 0 {
                ix
            } else {
                ix - 1
            }
        }
        fn get_elements_to_check_accross(ix: usize) -> usize {
            // Note: if let expression (=pattern matching)
            if let 0 | 9 = ix {
                2
            } else {
                3
            }
        }
        fn get_elements_to_check_alongside(ix: usize, ship_length: usize) -> usize {
            ship_length + if ix == 0 || ix + ship_length == BOARD_SIDE_LENGTH { 1 } else { 2 }
        }

        let number_of_rows_to_check = match direction {
            Direction::Horizontal => get_elements_to_check_accross(ix.row()),
            Direction::Vertical => get_elements_to_check_alongside(ix.row(), ship_length),
        };
        let number_of_cols_to_check = match direction {
            Direction::Horizontal => get_elements_to_check_alongside(ix.column(), ship_length),
            Direction::Vertical => get_elements_to_check_accross(ix.column()),
        };
        let first_check_row = get_first(ix.row());
        let first_check_col = get_first(ix.column());

        // Discuss: ranges with 1..100 and 1..=100
        for r in first_check_row..(first_check_row + number_of_rows_to_check) {
            for c in first_check_col..(first_check_col + number_of_cols_to_check) {
                if !self.can_place_ship(BoardIndex::from_col_row(c, r)) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn get_dummy_checker() -> Box<dyn PlacementCheck> {
        let mut mock = Box::new(MockPlacementCheck::new());
        mock.expect_can_place_ship().return_const(false);
        mock as Box<dyn PlacementCheck>
    }

    #[test]
    fn invalid_ship_length() {
        assert_eq!(
            ErrorKind::ShipTooLong,
            get_dummy_checker()
                .try_place_ship(BoardIndex::from_col_row(9, 0), 11, Direction::Horizontal)
                .err()
                .unwrap()
                .error_kind
        );
    }

    #[test]
    fn invalid_col() {
        assert_eq!(
            ErrorKind::OutOfBounds,
            get_dummy_checker()
                .try_place_ship(BoardIndex::from_col_row(9, 0), 2, Direction::Horizontal)
                .err()
                .unwrap()
                .error_kind
        );
    }

    #[test]
    fn invalid_row() {
        assert_eq!(
            ErrorKind::OutOfBounds,
            get_dummy_checker()
                .try_place_ship(BoardIndex::from_col_row(0, 9), 2, Direction::Vertical)
                .err()
                .unwrap()
                .error_kind
        );
    }

    #[rstest]
    #[case(0, 0, Direction::Horizontal, 0, 2, 0, 1)] // Left upper corner
    #[case(0, 0, Direction::Vertical, 0, 1, 0, 2)]
    #[case(8, 9, Direction::Horizontal, 7, 9, 8, 9)] // Right lower corner
    #[case(9, 8, Direction::Vertical, 8, 9, 7, 9)]
    #[case(0, 1, Direction::Horizontal, 0, 2, 0, 3)] // Left border
    #[case(0, 1, Direction::Vertical, 0, 1, 0, 3)]
    #[case(8, 1, Direction::Horizontal, 7, 9, 0, 2)] // Right border
    #[case(1, 8, Direction::Vertical, 0, 2, 7, 9)]
    #[case(1, 1, Direction::Horizontal, 0, 3, 0, 3)] // Middle (no border)
    #[case(1, 1, Direction::Vertical, 0, 3, 0, 3)]
    fn coordinate_check(
        #[case] col: usize,
        #[case] row: usize,
        #[case] direction: Direction,
        #[case] min_col: usize,
        #[case] max_col: usize,
        #[case] min_row: usize,
        #[case] max_row: usize,
    ) {
        let mut mock = MockPlacementCheck::new();
        mock.expect_can_place_ship().returning(move |ix| { 
            assert!(ix.column() >= min_col && ix.column() <= max_col);
            assert!(ix.row() >= min_row && ix.row() <= max_row);
            true
        });
        (&mock as &dyn PlacementCheck).try_place_ship(BoardIndex::from_col_row(col, row), 2, direction).unwrap();
    }
}
