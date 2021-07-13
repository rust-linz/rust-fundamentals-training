use std::ops::IndexMut;

use crate::{BOARD_SIDE_LENGTH, BoardIndex, Direction, SquareContent};

/*
    Learnings in this module:

    * Implement a trait based on another trait
    * Custom error type
    * Local functions
    * Mock objects for unit testing

    Recommended readings for this module:

    * mockall crate: https://github.com/asomers/mockall
    * Defining error types: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
    * Functions: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
*/

#[cfg_attr(test, mockall::automock)]
pub trait SquareAccessor {
    fn is_free(&self, ix: BoardIndex) -> bool;
    fn set_ship(&mut self, ix: BoardIndex);
}

impl<T> SquareAccessor for T where T: IndexMut<BoardIndex, Output = SquareContent> {
    fn is_free(&self, ix: BoardIndex) -> bool {
        self[ix] == SquareContent::Water
    }
    fn set_ship(&mut self, ix: BoardIndex) {
        self[ix] = SquareContent::Ship;
    }
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

pub trait ShipSetter {
    fn can_place_ship(
        &self,
        ix: BoardIndex,
        ship_length: usize,
        direction: Direction,
    ) -> Result<bool, PlacementError>;
    fn try_place_ship(
        &mut self,
        ix: BoardIndex,
        ship_length: usize,
        direction: Direction,
    ) -> Result<bool, PlacementError>;
}

impl<T> ShipSetter for T where T: SquareAccessor {
    fn can_place_ship(
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
                if !self.is_free(BoardIndex::from_col_row(c, r)) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    fn try_place_ship(
        &mut self,
        ix: BoardIndex,
        ship_length: usize,
        direction: Direction,
    ) -> Result<bool, PlacementError> {
        match self.can_place_ship(ix, ship_length, direction) {
            Ok(true) => {
                let mut ix = ix;
                for _ in 0..ship_length {
                    self.set_ship(ix);
                    ix = ix.try_next(direction).unwrap();
                }
                Ok(true)
            },
            res => res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::GenericBoardContent;

    use super::*;
    use rstest::rstest;

    fn get_dummy_free_checker() -> impl SquareAccessor {
        MockSquareAccessor::new()
    }

    #[test]
    fn invalid_ship_length() {
        assert_eq!(
            ErrorKind::ShipTooLong,
            get_dummy_free_checker()
                .can_place_ship(BoardIndex::from_col_row(9, 0), 11, Direction::Horizontal)
                .err()
                .unwrap()
                .error_kind
        );
    }

    #[test]
    fn invalid_col() {
        assert_eq!(
            ErrorKind::OutOfBounds,
            get_dummy_free_checker()
                .can_place_ship(BoardIndex::from_col_row(9, 0), 2, Direction::Horizontal)
                .err()
                .unwrap()
                .error_kind
        );
    }

    #[test]
    fn invalid_row() {
        assert_eq!(
            ErrorKind::OutOfBounds,
            get_dummy_free_checker()
                .can_place_ship(BoardIndex::from_col_row(0, 9), 2, Direction::Vertical)
                .err()
                .unwrap()
                .error_kind
        );
    }

    #[test]
    fn calls_is_free_correct_number_of_times()
    {
        let mut mock = MockSquareAccessor::new();
        mock.expect_is_free().times((3 + 1) * 2).return_const(true);
        assert!(mock.can_place_ship(BoardIndex::new(), 3, Direction::Horizontal).unwrap());
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
        let mut mock = MockSquareAccessor::new();
        mock.expect_is_free().returning(move |ix| { 
            // Discuss: Why can't we use a range pattern check here?
            assert!(ix.column() >= min_col && ix.column() <= max_col);
            assert!(ix.row() >= min_row && ix.row() <= max_row);
            true
        });
        assert!(mock.can_place_ship(BoardIndex::from_col_row(col, row), 2, direction).unwrap());
    }

    #[test]
    fn can_place_ship_overlap() {
        let mut mock = MockSquareAccessor::new();
        mock.expect_is_free().returning(|ix| { 
            !(ix.row() == 3 && ix.column() == 3)
        });
        assert!(!mock.can_place_ship(BoardIndex::from_col_row(3, 2), 3, Direction::Vertical).unwrap());
    }

    #[test]
    fn can_place_ship_on_board() {
        let board = GenericBoardContent::new_initialized(SquareContent::Water);
        assert!(board.can_place_ship("A1".parse().unwrap(), 2, Direction::Horizontal).unwrap());
    }

    #[test]
    fn try_place_ship_returns_error() {
        let mut board = GenericBoardContent::new_initialized(SquareContent::Water);
        board[BoardIndex::from_index(1)] = SquareContent::Ship;
        assert!(!board.try_place_ship(0.into(), 2, Direction::Horizontal).unwrap());
    }

    #[rstest]
    #[case(Direction::Vertical)]
    #[case(Direction::Horizontal)]
    fn try_place_ship_horizontal(#[case] direction: Direction) {
        let mut board = GenericBoardContent::new_initialized(SquareContent::Water);
        assert!(board.try_place_ship(0.into(), 2, direction).unwrap());
        assert_eq!(SquareContent::Ship, board[BoardIndex::from_index(0)]);
        assert_eq!(SquareContent::Ship, board[BoardIndex::from_index(0).try_next(direction).unwrap()]);
        assert_eq!(2, board.iter().filter(|s| { *s == SquareContent::Ship }).count());
        assert_eq!(98, board.into_iter().filter(|s| { *s == SquareContent::Water }).count());
    }
}
