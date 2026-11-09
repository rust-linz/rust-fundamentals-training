#![allow(dead_code)] // Note: To avoid warnings for unused code in this demo file

use std::{
    fmt,
    ops::{Index, IndexMut},
};

use crate::square_content::{InvalidSquareContent, SquareContent};

/// Number of rows and columns of the board
pub const BOARD_SIDE_LENGTH: usize = 3;

/// Total number of squares on the board
pub const BOARD_SQUARE_COUNT: usize = BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH;

#[derive(Debug, Clone, Copy)]
pub struct BoardContent {
    board_content: [Option<SquareContent>; BOARD_SQUARE_COUNT],
}

// Note: Implementation of BoardContent is not relevant for the demo of traits. We just
//       need some code for a meaningful demo.
impl BoardContent {
    pub fn new() -> Self {
        BoardContent::new_initialized(Default::default())
    }

    pub fn new_initialized(initial_content: Option<SquareContent>) -> BoardContent {
        BoardContent {
            board_content: [initial_content; BOARD_SQUARE_COUNT],
        }
    }

    pub fn as_slice(&self) -> &[Option<SquareContent>] {
        &self.board_content
    }
}

// Learning: Create our own trait and implement it for a struct
pub trait ToCompactString {
    fn to_compact_str(&self) -> String;
}

impl ToCompactString for BoardContent {
    fn to_compact_str(&self) -> String {
        let mut result = String::new();
        result.reserve_exact(BOARD_SQUARE_COUNT);

        // Note: Arrays implement IntoIterator, so we can iterate over them by value
        for c in self.board_content {
            result.push(match c {
                Some(c) => c.into(),
                None => ' ',
            });
        }

        result
    }
}

// Question: Can we implement the to_compact_str function in a more abstract way?
//           Can we make it applicable for any array of proper size of items that
//           support conversion to char?
// Learning: Yes, we can. We can implement our own trait for the array's IntoIter trait.
pub trait BoardIterToCompactString {
    fn to_compact_str(&mut self) -> String;
}

impl<T: Into<char>> BoardIterToCompactString
    for core::array::IntoIter<Option<T>, BOARD_SQUARE_COUNT>
{
    fn to_compact_str(&mut self) -> String {
        let mut result = String::new();
        result.reserve_exact(BOARD_SQUARE_COUNT);

        for c in self {
            result.push(match c {
                Some(c) => c.into(),
                None => ' ',
            });
        }

        result
    }
}

// Question: What's missing? We need to make our BoardContent an iterable.
// Learning: How to implement IntoIterator for our own struct
impl IntoIterator for BoardContent {
    type Item = Option<SquareContent>;

    type IntoIter = core::array::IntoIter<Self::Item, BOARD_SQUARE_COUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.board_content.into_iter()
    }
}

impl Default for BoardContent {
    fn default() -> Self {
        Self::new()
    }
}

// Learning: An enum can model the different reasons why an operation fails
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InvalidBoardData {
    WrongSize,
    InvalidSquare(InvalidSquareContent),
}

// Learning: Apply trait to an error type
impl fmt::Display for InvalidBoardData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidBoardData::WrongSize => write!(
                f,
                "Board data is of invalid size. Has to contain {BOARD_SQUARE_COUNT} elements"
            ),
            InvalidBoardData::InvalidSquare(e) => write!(f, "Board data is invalid: {e}"),
        }
    }
}

impl std::error::Error for InvalidBoardData {}

// Learning: This conversion is what makes the `?` operator work in `try_from` below
impl From<InvalidSquareContent> for InvalidBoardData {
    fn from(e: InvalidSquareContent) -> Self {
        InvalidBoardData::InvalidSquare(e)
    }
}

impl TryFrom<&[u8]> for BoardContent {
    // Note: We need the error type for implementing TryFrom.
    type Error = InvalidBoardData;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != BOARD_SQUARE_COUNT {
            return Err(InvalidBoardData::WrongSize);
        }

        let mut content = BoardContent::new();
        for (ix, square_byte) in bytes.iter().copied().enumerate() {
            content.board_content[ix] = match square_byte {
                0 => None,
                _ => Some(SquareContent::try_from(square_byte)?),
            };
        }

        Ok(content)
    }
}

impl From<BoardContent> for [u8; BOARD_SQUARE_COUNT] {
    fn from(c: BoardContent) -> Self {
        let mut content: [u8; BOARD_SQUARE_COUNT] = [0; BOARD_SQUARE_COUNT];
        for (ix, square) in c.board_content.iter().copied().enumerate() {
            content[ix] = match square {
                Some(c) => c.into(),
                None => 0,
            };
        }

        content
    }
}

// Learning: We can implement the Index trait to allow read-only indexing
impl Index<usize> for BoardContent {
    type Output = Option<SquareContent>;

    fn index(&self, ix: usize) -> &Self::Output {
        if ix >= BOARD_SQUARE_COUNT {
            panic!("Index out of bounds");
        }

        &self.board_content[ix]
    }
}

// Learning: We can implement the IndexMut trait to allow write indexing
impl IndexMut<usize> for BoardContent {
    fn index_mut(&mut self, ix: usize) -> &mut Self::Output {
        if ix >= BOARD_SQUARE_COUNT {
            panic!("Index out of bounds");
        }

        &mut self.board_content[ix]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let b = BoardContent::new();

        // Note: Here we make use of our IntoIterator implementation
        assert_eq!(b.into_iter().count(), BOARD_SQUARE_COUNT);
        assert!(b.into_iter().all(|v| v == Default::default()));
    }

    #[test]
    fn new_initialized() {
        let square_content = Some(SquareContent::X);
        let b = BoardContent::new_initialized(square_content);
        assert_eq!(b.into_iter().count(), BOARD_SQUARE_COUNT);
        assert!(b.into_iter().all(|v| v == square_content));
    }

    #[test]
    #[expect(
        clippy::clone_on_copy,
        reason = "this test exercises the derived Clone"
    )]
    fn clone() {
        let b = BoardContent::new_initialized(Some(SquareContent::X));
        let b2 = b.clone();
        assert_eq!(Some(SquareContent::X), b2[0]);
    }

    #[test]
    fn from_bytes() {
        let square_content = SquareContent::X;
        let content: &[u8] = &[square_content.into(); BOARD_SQUARE_COUNT];
        let board = BoardContent::try_from(content).unwrap();
        assert_eq!(board.into_iter().count(), BOARD_SQUARE_COUNT);
        assert!(board.into_iter().all(|v| v == Some(square_content)));
    }

    #[test]
    fn from_invalid_bytes() {
        let content: &[u8] = &[0; 99];
        let board = BoardContent::try_from(content);
        assert_eq!(Some(InvalidBoardData::WrongSize), board.err());
    }

    #[test]
    fn from_bytes_with_invalid_square() {
        let content: &[u8] = &[3; BOARD_SQUARE_COUNT];
        let board = BoardContent::try_from(content);
        assert_eq!(
            Some(InvalidBoardData::InvalidSquare(InvalidSquareContent)),
            board.err()
        );
    }

    #[test]
    fn into_bytes() {
        let board = BoardContent::new();
        let bytes: [u8; BOARD_SQUARE_COUNT] = board.into();
        assert_eq!(bytes.iter().count(), BOARD_SQUARE_COUNT);
        assert!(bytes.iter().all(|v| *v == 0));
    }

    #[test]
    #[should_panic]
    fn indexing_out_of_bounds() {
        let _ = BoardContent::new()[100];
    }

    #[test]
    #[should_panic]
    fn indexing_mut_out_of_bounds() {
        BoardContent::new()[100] = Option::None;
    }

    #[test]
    fn indexing() {
        let mut b = BoardContent::new();
        let square_content = Some(SquareContent::X);

        // Note: Here we make use of our implementation of read/write indexing
        b[8] = square_content;
        assert_eq!(square_content, b[8]);
    }

    #[test]
    fn slice_data() {
        let b = BoardContent::new();
        let bs = b.as_slice();
        assert_eq!(bs.len(), BOARD_SQUARE_COUNT);
    }

    #[test]
    fn into_short_str() {
        let b = BoardContent::new();

        // Note: Here we call the to_compact_str implemented on BoardContent
        assert_eq!(format!("{: >9}", ""), b.to_compact_str());

        // Note: Here we call the to_compact_str implemented on an iterator
        assert_eq!(format!("{: >9}", ""), b.into_iter().to_compact_str());
    }
}
