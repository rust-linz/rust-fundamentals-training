use std::ops::Index;

use crate::{BOARD_SIDE_LENGTH, GenericBoardContent};

/*
    Learnings in this module:

    * Lifetimes
    * Slices
    * Implementing custom iterators

    Recommended readings for this module:

    * Lifetimes: https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
    * Slice type: https://doc.rust-lang.org/book/ch04-03-slices.html
    * Iterators: https://doc.rust-lang.org/rust-by-example/trait/iter.html
*/

pub struct Row<'a, T> {
    // Note: Lifetime annotation here means that the struct `Row`
    // cannot outlive the reference it hold in the `board` field.
    board: &'a GenericBoardContent<T>,
    pub row_index: usize,
}

impl<'a, T: Default + Copy> Row<'a, T> {
    pub fn new(board: &'a GenericBoardContent<T>, row: usize) -> Self {
        Row { board, row_index: row }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.board.as_slice()[(self.row_index * BOARD_SIDE_LENGTH)..((self.row_index + 1) * BOARD_SIDE_LENGTH)]
    }

    pub fn has_next(&self) -> bool {
        self.row_index < (BOARD_SIDE_LENGTH - 1)
    }
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, ix: usize) -> &Self::Output {
        if ix >= BOARD_SIDE_LENGTH {
            panic!("Index out of bounds");
        }

        &self.board[self.row_index * BOARD_SIDE_LENGTH + ix]
    }
}

pub struct RowsIterator<'a, T> {
    board: &'a GenericBoardContent<T>,
    next_row: usize,
}

impl<'a, T> RowsIterator<'a, T> {
    pub fn new(board: &'a GenericBoardContent<T>) -> Self {
        RowsIterator { board, next_row: 0 }
    }
}

impl<'a, T> Iterator for RowsIterator<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Row<'a, T>> {
        if self.next_row >= BOARD_SIDE_LENGTH {
            return None;
        }

        let current = self.next_row;
        self.next_row += 1;
        Some(Row { board: self.board, row_index: current })
    }
}

#[cfg(test)]
mod tests {
    use crate::{BattleshipBoardContent, SquareContent};

    use super::*;

    #[test]
    fn slice_row() {
        let mut b = BattleshipBoardContent::new();
        let square_content = SquareContent::Ship;
        b[BOARD_SIDE_LENGTH] = square_content;

        // Note that we need the following variable to extend the lifetime of 
        // the `Row` instance. Otherwise, we could not call `data()` as our row
        // would have been clean up before the call. So the following line would NOT work:
        // let rs = b.row(1).data(); // <<< DOES NOT COMPILE
        let row = b.row(1);
        let rs = row.as_slice();
        assert_eq!(rs.len(), BOARD_SIDE_LENGTH);
        assert_eq!(rs[0], square_content);
    }

    #[test]
    fn row_iterator() {
        let mut b = BattleshipBoardContent::new();
        let square_content = SquareContent::Ship;
        b[BOARD_SIDE_LENGTH] = square_content;
        assert_eq!(BOARD_SIDE_LENGTH, b.rows().count());
        assert_eq!(b.rows().nth(1).unwrap().as_slice()[0], square_content);
    }
}
