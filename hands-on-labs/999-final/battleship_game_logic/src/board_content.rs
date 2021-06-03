use std::{cmp::Ordering, convert::TryFrom, fmt, ops::{Index, IndexMut}};

/*
    Learnings in this module:

    *

    Recommended readings for this module:

    * Constants: https://doc.rust-lang.org/std/keyword.const.html
    * Structs: https://doc.rust-lang.org/book/ch05-00-structs.html
    * Arrays: https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type
    * Generic data types: https://doc.rust-lang.org/book/ch10-01-syntax.html
    * Naming conventions (e.g. constructor functions): https://rust-lang.github.io/api-guidelines/naming.html
    * Custom error types: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
    * Formatters: https://doc.rust-lang.org/std/fmt/index.html
    * `Display` trait: https://doc.rust-lang.org/std/fmt/trait.Display.html
    * `TryFrom` and `TryInto` traits: https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html
    * `impl` trait: https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
    * `Index` trait: https://doc.rust-lang.org/std/ops/trait.Index.html
    * `IndexMut` trait: https://doc.rust-lang.org/std/ops/trait.IndexMut.html
    * `std::cmp`: https://doc.rust-lang.org/std/cmp/enum.Ordering.html

*/

const BOARD_SIDE_LENGTH: usize = 10;
pub const BOARD_SIZE: usize = BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH;

#[derive(Debug)]
pub struct BoardContent<T> {
    board_content: [T; BOARD_SIZE],
}

impl<T: Default + Copy> BoardContent<T> {
    pub fn new() -> BoardContent<T> {
        BoardContent::new_initialized(Default::default())
    }

    pub fn new_initialized(initial_content: T) -> BoardContent<T> {
        BoardContent {
            board_content: [initial_content; BOARD_SIZE],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.board_content.iter()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InvalidBoardSize;

impl fmt::Display for InvalidBoardSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Board data is of invalid size. Has to contain {size} elements",
            size = BOARD_SIZE
        )
    }
}

impl<T: From<u8> + Default + Copy> TryFrom<&[u8]> for BoardContent<T> {
    type Error = InvalidBoardSize;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes.len() {
            BOARD_SIZE => {
                let mut content = BoardContent::<T>::new();
                for (ix, square_byte) in bytes.iter().copied().enumerate() {
                    content.board_content[ix] = T::from(square_byte);
                }

                Ok(content)
            }
            _ => Err(InvalidBoardSize),
        }
    }
}

impl<T: Default + Into<u8> + Copy> Into<[u8; BOARD_SIZE]> for BoardContent<T> {
    fn into(self) -> [u8; BOARD_SIZE] {
        let mut content: [u8; BOARD_SIZE] = [T::default().into(); BOARD_SIZE];
        for (ix, square) in self.board_content.iter().copied().enumerate() {
            content[ix] = T::into(square);
        }

        content
    }
}

impl<T> Index<usize> for BoardContent<T> {
    type Output = T;

    fn index(&self, ix: usize) -> &Self::Output {
        match ix.cmp(&BOARD_SIZE) {
            Ordering::Greater | Ordering::Equal => panic!("Index out of bounds"),
            _ => &self.board_content[ix],
        }
    }
}

impl<T> IndexMut<usize> for BoardContent<T> {
    fn index_mut(&mut self, ix: usize) -> &mut Self::Output {
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
        let b = BoardContent::<SquareContent>::new();
        assert_eq!(b.iter().count(), 100);
        assert!(b.iter().all(|v| *v == Default::default()));
    }

    #[test]
    fn new_initialized() {
        let square_content = SquareContent::Ship;
        let b = BoardContent::new_initialized(square_content);
        assert_eq!(b.iter().count(), 100);
        assert!(b.iter().all(|v| *v == square_content));
    }

    #[test]
    fn from_bytes() {
        let square_content = SquareContent::Ship;
        let content: &[u8] = &[square_content.into(); BOARD_SIZE];
        let board = BoardContent::<SquareContent>::try_from(content).unwrap();
        assert_eq!(board.iter().count(), 100);
        assert!(board.iter().all(|v| *v == square_content));
    }

    #[test]
    fn from_invalid_bytes() {
        let content: &[u8] = &[0; 99];
        let board = BoardContent::<SquareContent>::try_from(content);
        assert_eq!(Some(InvalidBoardSize), board.err());
    }

    #[test]
    fn into_bytes() {
        let board = BoardContent::<SquareContent>::new();
        let bytes: [u8; BOARD_SIZE] = board.into();
        assert_eq!(bytes.iter().count(), 100);
        assert!(bytes.iter().all(|v| *v == SquareContent::default().into()));
    }

    #[test]
    #[should_panic]
    fn indexing_out_of_bounds() {
        BoardContent::<SquareContent>::new()[100];
    }

    #[test]
    #[should_panic]
    fn indexing_mut_out_of_bounds() {
        BoardContent::<SquareContent>::new()[100] = SquareContent::SunkenShip;
    }

    #[test]
    fn indexing() {
        let mut b = BoardContent::<SquareContent>::new();
        let square_content = SquareContent::Ship;
        b[99] = square_content;
        assert_eq!(square_content, b[99]);
    }
}
