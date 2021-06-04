use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt,
    ops::{Index, IndexMut},
};

use crate::{BOARD_SIDE_LENGTH, BOARD_SIZE, Row, RowsIterator, SquareContent};

/*
    Learnings in this module:

    * Create a generic type
    * Work with custom error types
    * Fundamentals of iterators

    Recommended readings for this module:

    * Generic data types: https://doc.rust-lang.org/book/ch10-01-syntax.html
    * Custom error types: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
    * Formatters: https://doc.rust-lang.org/std/fmt/index.html
    * `Display` trait: https://doc.rust-lang.org/std/fmt/trait.Display.html
    * `TryFrom` and `TryInto` traits: https://doc.rust-lang.org/rust-by-example/conversion/try_from_try_into.html
    * `impl` trait: https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
*/

pub type BattleshipBoardContent = GenericBoardContent<SquareContent>;

#[derive(Debug)]
pub struct GenericBoardContent<T> {
    board_content: [T; BOARD_SIZE],
}

impl<T: Default + Copy> GenericBoardContent<T> {
    pub fn new() -> Self {
        GenericBoardContent::new_initialized(Default::default())
    }

    pub fn new_initialized(initial_content: T) -> GenericBoardContent<T> {
        GenericBoardContent {
            board_content: [initial_content; BOARD_SIZE],
        }
    }

    pub fn into_iter(&self) -> impl Iterator<Item = T> {
        core::array::IntoIter::new(self.board_content)
    }

    pub fn row(&self, row: usize) -> Row<T> {
        if row >= BOARD_SIDE_LENGTH {
            panic!("Index out of bounds");
        }

        Row::new(self, row)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.board_content
    }

    pub fn rows(&self) -> impl Iterator<Item = Row<T>> {
        RowsIterator::new(self)
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

impl<T: From<u8> + Default + Copy> TryFrom<&[u8]> for GenericBoardContent<T> {
    type Error = InvalidBoardSize;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes.len() {
            BOARD_SIZE => {
                let mut content = GenericBoardContent::<T>::new();
                for (ix, square_byte) in bytes.iter().copied().enumerate() {
                    content.board_content[ix] = T::from(square_byte);
                }

                Ok(content)
            }
            _ => Err(InvalidBoardSize),
        }
    }
}

impl<T: Default + Into<u8> + Copy> Into<[u8; BOARD_SIZE]> for GenericBoardContent<T> {
    fn into(self) -> [u8; BOARD_SIZE] {
        let mut content: [u8; BOARD_SIZE] = [T::default().into(); BOARD_SIZE];
        for (ix, square) in self.board_content.iter().copied().enumerate() {
            content[ix] = T::into(square);
        }

        content
    }
}

impl<T> Index<usize> for GenericBoardContent<T> {
    type Output = T;

    fn index(&self, ix: usize) -> &Self::Output {
        match ix.cmp(&BOARD_SIZE) {
            Ordering::Greater | Ordering::Equal => panic!("Index out of bounds"),
            _ => &self.board_content[ix],
        }
    }
}

impl<T> IndexMut<usize> for GenericBoardContent<T> {
    fn index_mut(&mut self, ix: usize) -> &mut Self::Output {
        if ix >= BOARD_SIZE {
            panic!("Index out of bounds");
        }

        &mut self.board_content[ix]
    }
}

impl<T: Into<char> + Default + Copy> Into<String> for GenericBoardContent<T> {
    fn into(self) -> String {
        fn build_separator(chars: &[char]) -> String {
            let mut result = String::new();
            result.reserve_exact(
                1 * chars[0].len_utf8()
                    + BOARD_SIDE_LENGTH * 2 * chars[1].len_utf8()
                    + (BOARD_SIDE_LENGTH - 1) * chars[2].len_utf8()
                    + 1 * chars[3].len_utf8()
                    + 1 * '\n'.len_utf8(),
            );
            result.push(chars[0]);
            for _ in 0..BOARD_SIDE_LENGTH - 1 {
                result.push(chars[1]);
                result.push(chars[1]);
                result.push(chars[2]);
            }

            result.push(chars[1]);
            result.push(chars[1]);
            result.push(chars[3]);

            result.push('\n');

            result
        }

        let top = build_separator(&['┏', '━', '┯', '┓']);
        let middle = build_separator(&['┠', '─', '┼', '┨']);
        let bottom = build_separator(&['┗', '━', '┷', '┛']);

        let mut result = String::new();
        result.reserve_exact(
            top.len()
                + middle.len() * (BOARD_SIDE_LENGTH - 1)
                + bottom.len()
                + (2 * '┃'.len_utf8()
                    + (BOARD_SIDE_LENGTH - 1) * '|'.len_utf8()
                    + BOARD_SIDE_LENGTH * 2 * ' '.len_utf8()
                    + '\n'.len_utf8())
                    * BOARD_SIDE_LENGTH,
        );

        result.push_str(&top);

        for row in self.rows() {
            result.push('┃');
            for col in 0..BOARD_SIDE_LENGTH {
                let char = row[col].into();
                result.push(char);
                result.push(char);
                if col < 9 {
                    result.push('|');
                }
            }

            result.push('┃');
            result.push('\n');
            if row.row_index < 9 {
                result.push_str(&middle);
            }
        }

        result.push_str(&bottom);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::SquareContent;

    use super::*;

    #[test]
    fn new() {
        let b = BattleshipBoardContent::new();
        assert_eq!(b.into_iter().count(), 100);
        assert!(b.into_iter().all(|v| v == Default::default()));
    }

    #[test]
    fn new_initialized() {
        let square_content = SquareContent::Ship;
        let b = BattleshipBoardContent::new_initialized(square_content);
        assert_eq!(b.into_iter().count(), 100);
        assert!(b.into_iter().all(|v| v == square_content));
    }

    #[test]
    fn from_bytes() {
        let square_content = SquareContent::Ship;
        let content: &[u8] = &[square_content.into(); BOARD_SIZE];
        let board = BattleshipBoardContent::try_from(content).unwrap();
        assert_eq!(board.into_iter().count(), 100);
        assert!(board.into_iter().all(|v| v == square_content));
    }

    #[test]
    fn from_invalid_bytes() {
        let content: &[u8] = &[0; 99];
        let board = BattleshipBoardContent::try_from(content);
        assert_eq!(Some(InvalidBoardSize), board.err());
    }

    #[test]
    fn into_bytes() {
        let board = BattleshipBoardContent::new();
        let bytes: [u8; BOARD_SIZE] = board.into();
        assert_eq!(bytes.iter().count(), 100);
        assert!(bytes.iter().all(|v| *v == SquareContent::default().into()));
    }

    #[test]
    #[should_panic]
    fn indexing_out_of_bounds() {
        BattleshipBoardContent::new()[100];
    }

    #[test]
    #[should_panic]
    fn indexing_mut_out_of_bounds() {
        BattleshipBoardContent::new()[100] = SquareContent::SunkenShip;
    }

    #[test]
    fn indexing() {
        let mut b = BattleshipBoardContent::new();
        let square_content = SquareContent::Ship;
        b[99] = square_content;
        assert_eq!(square_content, b[99]);
    }

    #[test]
    fn slice_data() {
        let b = BattleshipBoardContent::new();
        let bs = b.as_slice();
        assert_eq!(bs.len(), BOARD_SIZE);
    }

    #[test]
    fn into_string() {
        let b = BattleshipBoardContent::new();
        let _: String = b.into();
    }
}
