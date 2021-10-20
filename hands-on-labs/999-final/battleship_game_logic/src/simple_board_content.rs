use std::{cmp::Ordering, ops::{Index, IndexMut}};

use crate::{BOARD_SIZE, SquareContent};

/*
    Learnings in this module:

    * Build a simple struct
    * Learn how traits are used to implement operators (in this case indexer)

    Recommended readings for this module:

    * Naming conventions (e.g. constructor functions): https://rust-lang.github.io/api-guidelines/naming.html
    * Structs: https://doc.rust-lang.org/book/ch05-00-structs.html
    * Arrays: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type
    * `Index` trait: https://doc.rust-lang.org/std/ops/trait.Index.html
    * `IndexMut` trait: https://doc.rust-lang.org/std/ops/trait.IndexMut.html
    * `std::cmp`: https://doc.rust-lang.org/std/cmp/enum.Ordering.html
*/

#[derive(Debug)]
pub struct SimpleBoardContent {
    board_content: [SquareContent; BOARD_SIZE],
}

impl SimpleBoardContent {
    pub fn new() -> Self {
        SimpleBoardContent::new_initialized(Default::default())
    }

    pub fn new_initialized(initial_content: SquareContent) -> Self {
        SimpleBoardContent {
            board_content: [initial_content; BOARD_SIZE],
        }
    }
}

impl Default for SimpleBoardContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for SimpleBoardContent {
    type Output = SquareContent;

    fn index(&self, ix: usize) -> &Self::Output {
        // Compare with std::cmp
        match ix.cmp(&BOARD_SIZE) {
            Ordering::Greater | Ordering::Equal => panic!("Index out of bounds"),
            _ => &self.board_content[ix],
        }
    }
}

impl IndexMut<usize> for SimpleBoardContent {
    fn index_mut(&mut self, ix: usize) -> &mut Self::Output {
        // Compare with if
        if ix >= BOARD_SIZE {
            panic!("Index out of bounds");
        }

        &mut self.board_content[ix]
    }
}

#[cfg(test)]
mod tests {
    use crate::SquareContent;

    use super::*;

    #[test]
    fn new() {
        let _b = SimpleBoardContent::new();
    }

    #[test]
    fn new_initialized() {
        let square_content = SquareContent::Ship;
        let _b = SimpleBoardContent::new_initialized(square_content);
    }

    #[test]
    #[should_panic]
    fn indexing_out_of_bounds() {
        SimpleBoardContent::new()[100];
    }

    #[test]
    #[should_panic]
    fn indexing_mut_out_of_bounds() {
        SimpleBoardContent::new()[100] = SquareContent::SunkenShip;
    }

    #[test]
    fn indexing() {
        let mut b = SimpleBoardContent::new();
        let square_content = SquareContent::Ship;
        b[99] = square_content;
        assert_eq!(square_content, b[99]);
    }
}
