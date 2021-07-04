use crate::{BoardIndex, Direction, ShipSetter};

/*
    Learnings in this module:

    * Working with random numbers
    * Mocking of external traits

    Recommended readings for this module:

    * Function pointer types: https://doc.rust-lang.org/reference/types/function-pointer.html
    * rand crate: https://crates.io/crates/rand
    * Mockall - mocking external traits: https://docs.rs/mockall/0.10.1/mockall/#external-traits
    * Mockall - call counting: https://docs.rs/mockall/0.10.1/mockall/#callcounts
*/

pub trait BoardFiller {
    fn place_ship(&mut self, ship: &usize);
    fn fill_random(&mut self, ship_lengths: &[usize]);
}

impl<T> BoardFiller for T where T: ShipSetter {
    fn fill_random(&mut self, ship_lengths: &[usize]) {
        for ship_length in ship_lengths {
            self.place_ship(ship_length);
        }
    }

    fn place_ship(&mut self, ship: &usize) {
        for _ in 0..1000 {
            let direction: Direction;
            let col: usize;
            let row: usize;
            match rand::random::<usize>() % 2 == 0 {
                true => {
                    direction = Direction::Horizontal;
                    col = rand::random::<usize>() % (10 - ship);
                    row = rand::random::<usize>() % 10;
                }
                false => {
        
                direction = Direction::Vertical;
                    col = rand::random::<usize>() % 10;
                    row = rand::random::<usize>() % (10 - ship);
                }
            };

            if self.try_place_ship(BoardIndex::from_col_row(col, row), *ship, direction).unwrap() {
                return;
            }
        }

        panic!("Cannot position ships, board is too occupied.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use crate::{PlacementError};

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
        mock.fill_random(&[2, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn fill_failure() {
        let mut mock = MockMyFillableBoard::new();
        mock.expect_can_place_ship().return_const(Ok(false));
        mock.fill_random(&[2]);
    }
}
