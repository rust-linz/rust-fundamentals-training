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

    pub fn try_parse(location: &str) -> Result<BoardIndex, &'static str> {
        // Discuss difference between location.chars() and location.len()
        if location.len() != 2 {
            return Err("Invalid length");
        }

        let col = match location.chars().nth(0) {
            // Check experimental `if let` syntax
            Some(c) if matches!(c, 'A'..='J') => c as usize - 'A' as usize,
            _ => return Err("Invalid column"),
        };

        let row = match location.chars().nth(1) {
            Some(c) if matches!(c, '0'..='9') => c as usize - '0' as usize,
            _ => return Err("Invalid column"),
        };

        Ok(BoardIndex::from_row_col(col, row))
    }
}

#[cfg(test)]
mod tests {
     use super::*;

     #[test]
     fn new() {
         BoardIndex::new();
     }

     #[test]
     fn from_index() {
        let ix = BoardIndex::from_index(0);
        assert_eq!(ix, BoardIndex::new());
     }

     #[test]
     fn from_row_col() {
        let ix = BoardIndex::from_row_col(0, 0);
        assert_eq!(ix, BoardIndex::new());
     }

     #[test]
     fn try_parse() {
        let ix = BoardIndex::try_parse("A0").unwrap();
        assert_eq!(ix, BoardIndex::new());
     }

     #[test]
     fn try_parse_invalid_col() {
        let ix = BoardIndex::try_parse("K0");
        assert!(ix.is_err());
     }

     #[test]
     fn try_parse_invalid_row() {
        let ix = BoardIndex::try_parse("KA");
        assert!(ix.is_err());
     }

     #[test]
     fn try_parse_invalid_length() {
        let ix = BoardIndex::try_parse("A");
        assert!(ix.is_err());
     }
}