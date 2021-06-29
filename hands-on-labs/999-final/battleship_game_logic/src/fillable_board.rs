use std::ops::Index;

use mockall::automock;

use crate::{BOARD_SIDE_LENGTH, BoardIndex, SquareContent};

pub enum Direction {
    Horizontal,
    Vertical
}

#[automock]
pub trait PlacementCheck {
    fn can_place_ship(&self, ix: BoardIndex) -> bool;
}

impl PlacementCheck for dyn Index<usize, Output = SquareContent> {
    fn can_place_ship(&self, ix: BoardIndex) -> bool {
        self[ix.into()] == SquareContent::Water
    }
}

pub trait FillableBoard {
    fn try_place_ship(&self, ix: BoardIndex, ship_length: usize, direction: Direction) -> bool;
}

impl FillableBoard for dyn PlacementCheck {
    fn try_place_ship(&self, ix: BoardIndex, ship_length: usize, direction: Direction) -> bool {
        if ship_length > BOARD_SIDE_LENGTH {
            panic!("Maximum length of ship is 10");
        }

        fn outside_bounds(start: usize, ship_length: usize) -> bool {
            start + ship_length > 10
        }
        if outside_bounds(match direction {
            Direction::Vertical => ix.column(),
            Direction::Horizontal => ix.row(),
        }, ship_length) {
            return false;
        }

        fn get_first(ix: usize) -> usize {
            // Note: if expression (not statement)
            if ix == 0 { ix } else { ix - 1}
        }
        fn get_elements_to_check_accross(ix: usize) -> usize {
            // Note: if let expression (=pattern matching)
            if let 0 | 9 = ix { 2 } else { 3 }
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
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_dummy_checker() -> Box<dyn PlacementCheck> {
        let mut mock = Box::new(MockPlacementCheck::new());
        mock.expect_can_place_ship().return_once(|_| { false });
        mock as Box<dyn PlacementCheck>
    }

    #[test]
    #[should_panic]
    fn invalid_ship_length() {
        get_dummy_checker().try_place_ship(BoardIndex::new(), 11, Direction::Horizontal);
    }

    #[test]
    fn invalid_col() {
        assert!(!get_dummy_checker().try_place_ship(BoardIndex::from_col_row(9, 0), 2, Direction::Horizontal));
    }
}
