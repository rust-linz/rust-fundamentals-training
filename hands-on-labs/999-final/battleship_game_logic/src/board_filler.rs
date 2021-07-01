use crate::{BoardIndex, Direction, FillableBoard};

pub fn fill_random(ship_lengths: &[usize], board: &dyn FillableBoard) {
    for ship in ship_lengths {
        place_ship(ship, board);
    }
}

fn place_ship(ship: &usize, board: &dyn FillableBoard) {
    for _ in 0..1000 {
        let direction: Direction;
        let col: usize;
        let row: usize;
        match rand::random::<usize>() % 2 == 0 {
            true => {
                direction = Direction::Horizontal;
                col = rand::random::<usize>() % (9 - ship);
                row = rand::random::<usize>() % 9;
            }
            false => {
    
            direction = Direction::Vertical;
                col = rand::random::<usize>() % 9;
                row = rand::random::<usize>() % (9 - ship);
            }
        };

        if board.try_place_ship(BoardIndex::from_col_row(col, row), *ship, direction).unwrap() {
            return;
        }
    }

    panic!("Cannot position ships, board is too occupied.");
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use crate::PlacementError;

    mock! {
        MyFillableBoard {}
        impl FillableBoard for MyFillableBoard {
            fn try_place_ship(
                &self,
                ix: BoardIndex,
                ship_length: usize,
                direction: Direction,
            ) -> Result<bool, PlacementError>;
        }
    }

    #[test]
    fn fill() {
        let mut mock = MockMyFillableBoard::new();
        mock.expect_try_place_ship().times(3).return_const(Ok(true));
        fill_random(&[2, 3, 4], &mock as &dyn FillableBoard);
    }

    #[test]
    #[should_panic]
    fn fill_failure() {
        let mut mock = MockMyFillableBoard::new();
        mock.expect_try_place_ship().return_const(Ok(false));
        fill_random(&[2], &mock as &dyn FillableBoard);
    }
}
