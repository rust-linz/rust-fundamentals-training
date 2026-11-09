use crate::{BoardIndex, Direction, ShipSetter};

/*
    Learnings in this module:

    * Function pointer types
    * Working with random numbers
    * Mocking of external traits

    Recommended readings for this module:

    * Function pointer types: https://doc.rust-lang.org/reference/types/function-pointer.html
    * rand crate: https://crates.io/crates/rand
    * Mockall - mocking external traits: https://docs.rs/mockall/latest/mockall/#external-traits
    * Mockall - call counting: https://docs.rs/mockall/latest/mockall/#call-counts
*/

pub type ShipPlacer = fn(ship: usize, &mut dyn ShipSetter);

pub fn random_placer(ship: usize, setter: &mut dyn ShipSetter) {
    for _ in 0..1000 {
        let direction: Direction;
        let col: usize;
        let row: usize;
        // Note: `..=` because a ship may end exactly on the last column or row
        match rand::random::<bool>() {
            true => {
                direction = Direction::Horizontal;
                col = rand::random_range(0..=10 - ship);
                row = rand::random_range(0..10);
            }
            false => {
                direction = Direction::Vertical;
                col = rand::random_range(0..10);
                row = rand::random_range(0..=10 - ship);
            }
        };

        if setter.try_place_ship(BoardIndex::from_col_row(col, row), ship, direction).unwrap() {
            return;
        }
    }

    panic!("Cannot position ships, board is too occupied.");    
}

pub trait BoardFiller {
    fn fill(&mut self, ship_lengths: &[usize], placer: ShipPlacer);
}

impl<T> BoardFiller for T where T: ShipSetter {
    fn fill(&mut self, ship_lengths: &[usize], placer: ShipPlacer) {
        for ship_length in ship_lengths {
            placer(*ship_length, self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use crate::{BattleshipBoardContent, PlacementError, SquareContent};

    mock! {
        MyFillableBoard {}
        impl ShipSetter for MyFillableBoard {
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
            ) -> Result<bool, PlacementError>;        }
    }

    #[test]
    fn fill() {
        let mut mock = MockMyFillableBoard::new();
        mock.expect_try_place_ship().times(3).return_const(Ok(true));
        mock.fill(&[2, 3, 4], random_placer);
    }

    #[test]
    #[should_panic(expected = "Cannot position ships")]
    fn fill_failure() {
        let mut mock = MockMyFillableBoard::new();
        mock.expect_try_place_ship().return_const(Ok(false));
        mock.fill(&[2], random_placer);
    }

    #[test]
    fn random_placer_reaches_last_column_and_row() {
        // A ship of length 9 starts at position 0 or 1 along its axis. With 500 attempts,
        // missing either the last column or the last row is practically impossible.
        let (mut reached_last_column, mut reached_last_row) = (false, false);
        for _ in 0..500 {
            let mut board = BattleshipBoardContent::new_initialized(SquareContent::Water);
            random_placer(9, &mut board);

            let is_ship = |col, row| board[BoardIndex::from_col_row(col, row)] == SquareContent::Ship;
            reached_last_column |= (0..10).any(|row| is_ship(9, row) && is_ship(8, row));
            reached_last_row |= (0..10).any(|col| is_ship(col, 9) && is_ship(col, 8));
        }

        assert!(reached_last_column);
        assert!(reached_last_row);
    }
}
