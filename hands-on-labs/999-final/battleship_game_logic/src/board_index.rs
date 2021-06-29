use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::BOARD_SIDE_LENGTH;

/*
    Learnings in this module:

    * Building a new type based on a basic data type
    * Writing constructor functions
    * Writing functions for converting types (into, from)
    * String handling fundamentals
    * Writing unit tests (including data-driven tests with rstest)

    Recommended readings for this module:

    * newtype pattern: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
    * Strings: https://doc.rust-lang.org/rust-by-example/std/str.html
    * Operator overloading: https://doc.rust-lang.org/std/ops/index.html
    * rstest crate: https://docs.rs/rstest/0.10.0/rstest/index.html
*/

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BoardIndex(usize);

impl BoardIndex {
    pub fn new() -> BoardIndex {
        BoardIndex(0)
    }

    pub fn from_index(index: usize) -> BoardIndex {
        if index >= BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH {
            panic!("Index out of bounds");
        }

        BoardIndex(index)
    }

    pub fn from_col_row(col: usize, row: usize) -> BoardIndex {
        if col >= BOARD_SIDE_LENGTH {
            panic!("Column out of bounds");
        }

        if row >= BOARD_SIDE_LENGTH {
            panic!("Row out of bounds");
        }

        BoardIndex(row * BOARD_SIDE_LENGTH + col)
    }

    pub fn from_str<T: AsRef<str>>(location: T) -> BoardIndex {
        // Note that this method signature allows us to call method
        // with String and string slice (&str).
        BoardIndex::try_from_str(location).unwrap()
    }

    pub fn try_from_str<T: AsRef<str>>(location: T) -> Result<BoardIndex, &'static str> {
        let location = location.as_ref().as_bytes(); // Note shadowing

        // Check if length of location is ok (A1..J10).
        // Discuss difference between location.chars().count() and location.len()
        if !matches!(location.len(), 2..=3) {
            return Err("Invalid length");
        }

        // Parse column letter (A..J, a..j)
        let col = match location[0] {
            r if matches!(r, b'A'..=b'J') => (r - b'A') as usize, // Check experimental `if let` syntax
            r if matches!(r, b'a'..=b'j') => (r - b'a') as usize,
            _ => return Err("Invalid column"),
        };

        // Parse the row letter(s) (1..10)
        let row: usize; // Note: No mut here
        if location.len() == 3 {
            if location[1..] != [b'1', b'0'] {
                // Note slice pattern
                return Err("Invalid row");
            }

            row = 9;
        } else {
            row = match location[1] {
                c if matches!(c, b'1'..=b'9') => (c - b'1') as usize,
                _ => return Err("Invalid row"),
            };
        }

        Ok(BoardIndex::from_col_row(col, row))
    }

    pub fn column(&self) -> usize {
        self.0 % BOARD_SIDE_LENGTH
    }

    pub fn row(&self) -> usize {
        self.0 / BOARD_SIDE_LENGTH
    }
}

impl Into<usize> for BoardIndex {
    fn into(self) -> usize {
        self.0
    }
}

impl Into<String> for BoardIndex {
    // Discuss: Why can't you return &str here?
    // See also https://stackoverflow.com/a/29429698/3548127
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl From<usize> for BoardIndex {
    fn from(ix: usize) -> Self {
        BoardIndex::from_index(ix)
    }
}

impl From<&str> for BoardIndex {
    fn from(ix: &str) -> Self {
        BoardIndex::from_str(ix)
    }
}

impl Display for BoardIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            char::from_u32((b'A' as usize + self.0 / BOARD_SIDE_LENGTH) as u32).unwrap(),
            self.0 % BOARD_SIDE_LENGTH + 1
        )
    }
}

impl Add<usize> for BoardIndex {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        (self.0 + other).into()
    }
}

impl Sub<usize> for BoardIndex {
    type Output = Self;

    fn sub(self, other: usize) -> Self {
        (self.0 - other).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new() {
        let ix = BoardIndex::new();
        assert_eq!(0usize, ix.into());
    }

    #[test]
    fn from_index() {
        let ix = BoardIndex::from_index(10);
        assert_eq!(BOARD_SIDE_LENGTH, ix.into());
    }

    #[test]
    fn from_usize() {
        assert_eq!(BoardIndex::from_index(10), 10.into());
    }

    #[test]
    #[should_panic]
    fn from_index_out_of_bounds() {
        BoardIndex::from_index(BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH);
    }

    #[test]
    fn from_row_col() {
        let ix = BoardIndex::from_col_row(0, 1);
        assert_eq!(BOARD_SIDE_LENGTH, ix.into());
    }

    #[test]
    #[should_panic]
    fn from_row_col_row_out_of_bounds() {
        BoardIndex::from_col_row(0, BOARD_SIDE_LENGTH);
    }

    #[test]
    #[should_panic]
    fn from_row_col_col_out_of_bounds() {
        BoardIndex::from_col_row(BOARD_SIDE_LENGTH, 0);
    }

    #[test]
    fn from_str_lowest() {
        let ix = BoardIndex::from_str("A1");
        assert_eq!(0usize, ix.into());
    }

    #[test]
    fn from_str_highest() {
        let ix = BoardIndex::from_str("J10");
        assert_eq!(BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH - 1, ix.into());
    }

    #[test]
    fn from_str_lowercase() {
        let ix = BoardIndex::from_str("a1");
        assert_eq!(0usize, ix.into());
    }

    #[test]
    fn from_str_string() {
        BoardIndex::from_str(String::from("A1"));
    }

    #[test]
    fn from_str_slice() {
        BoardIndex::from_str("A1");
    }

    #[test]
    fn into_str() {
        let ix = BoardIndex::new();
        assert_eq!("A1", Into::<String>::into(ix));
    }

    #[test]
    fn into_index_from_str() {
        assert_eq!(BoardIndex::new(), "A1".into());
    }

    #[test]
    fn from_str() {
        assert_eq!(BoardIndex::new(), From::<&str>::from("A1"));
    }

    #[rstest]
    #[case("B01")] // leading zero
    #[case("B11")] // too high column
    #[case("B0")] // too low column
    #[case("B")] // missing column
    #[case("9")] // missing row
    #[case("J1A")] // invalid length
    #[case("AA10")] // invalid length
    #[case("99")] // missing row
    fn try_parse_errors(#[case] location: &'static str) {
        assert!(BoardIndex::try_from_str(location).is_err());
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(2, 1, 2)]
    #[case(9, 9, 9)]
    fn get_column(#[case] col: usize, #[case] row: usize, #[case] expected_col: usize) {
        assert_eq!(expected_col, BoardIndex::from_col_row(col, row).column());
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(2, 1, 1)]
    #[case(9, 9, 9)]
    fn get_row(#[case] col: usize, #[case] row: usize, #[case] expected_row: usize) {
        assert_eq!(expected_row, BoardIndex::from_col_row(col, row).row());
    }

    #[rstest]
    #[case(0, 0, "A1")]
    #[case(9, 9, "J10")]
    #[case(1, 1, "B2")]
    fn try_display(#[case] col: usize, #[case] row: usize, #[case] location: &'static str) {
        assert_eq!(location, format!("{}", BoardIndex::from_col_row(col, row)));
    }

    #[test]
    fn add() {
        assert_eq!(BoardIndex::from_index(2), BoardIndex::from_index(1) + 1);
    }

    #[test]
    fn sub() {
        assert_eq!(BoardIndex::from_index(0), BoardIndex::from_index(1) - 1);
    }

    #[test]
    #[should_panic]
    fn add_overflow() {
        let _ = BoardIndex::from_index(BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH - 1) + 1;
    }

    #[test]
    #[should_panic]
    fn sub_overflow() {
        let _ = BoardIndex::from_index(0) - 1;
    }
}
