use crate::BOARD_SIDE_LENGTH;

// Note the newtype pattern here. Read more at https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    pub fn from_row_col(col: usize, row: usize) -> BoardIndex {
        if col >= BOARD_SIDE_LENGTH {
            panic!("Column out of bounds");
        }

        if row >= BOARD_SIDE_LENGTH {
            panic!("Row out of bounds");
        }

        BoardIndex(row * 10 + col)
    }

    pub fn from_str(location: &str) -> Result<BoardIndex, &'static str> {
        let location = location.as_bytes(); // Note shadowing

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
            if location[1..] != [b'1', b'0'] { // Note slice pattern
                return Err("Invalid row");
            }

            row = 9;
        }
        else {
            row = match location[1] {
                c if matches!(c, b'1'..=b'9') => (c - b'1') as usize,
                _ => return Err("Invalid row"),
            };
        }

        Ok(BoardIndex::from_row_col(col, row))
    }
}

impl Into<usize> for BoardIndex {
    fn into(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let ix = BoardIndex::new();
        assert_eq!(0usize, ix.into());
    }

    #[test]
    fn from_index() {
        let ix = BoardIndex::from_index(10);
        assert_eq!(10usize, ix.into());
    }

    #[test]
    #[should_panic]
    fn from_index_out_of_bounds() {
        BoardIndex::from_index(100);
    }

    #[test]
    fn from_row_col() {
        let ix = BoardIndex::from_row_col(0, 1);
        assert_eq!(10usize, ix.into());
    }

    #[test]
    #[should_panic]
    fn from_row_col_row_out_of_bounds() {
        BoardIndex::from_row_col(0, 10);
    }

    #[test]
    #[should_panic]
    fn from_row_col_col_out_of_bounds() {
        BoardIndex::from_row_col(10, 0);
    }

    #[test]
    fn try_parse_lowest() {
        let ix = BoardIndex::from_str("A1").unwrap();
        assert_eq!(0usize, ix.into());
    }

    #[test]
    fn try_parse_highest() {
        let ix = BoardIndex::from_str("J10").unwrap();
        assert_eq!(99usize, ix.into());
    }

    #[test]
    fn try_parse_lowercase() {
        let ix = BoardIndex::from_str("a1").unwrap();
        assert_eq!(0usize, ix.into());
    }

    #[test]
    fn try_parse_errors() {
        assert!(BoardIndex::from_str("B01").is_err()); // leading zero
        assert!(BoardIndex::from_str("B11").is_err()); // too high column
        assert!(BoardIndex::from_str("B0").is_err()); // too low column
        assert!(BoardIndex::from_str("B").is_err()); // missing column
        assert!(BoardIndex::from_str("9").is_err()); // missing row
        assert!(BoardIndex::from_str("J1A").is_err()); // Invalid length
        assert!(BoardIndex::from_str("AA10").is_err()); // Invalid length
        assert!(BoardIndex::from_str("99").is_err()); // missing row
    }
}
