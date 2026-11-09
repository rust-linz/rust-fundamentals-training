use std::str::FromStr;

use crate::board_content::BOARD_SIDE_LENGTH;

#[derive(Copy, Clone)]
pub struct BoardIndex(usize);

impl BoardIndex {
    pub fn from_col_row(col: usize, row: usize) -> BoardIndex {
        if col >= BOARD_SIDE_LENGTH {
            panic!("Column out of bounds");
        }

        if row >= BOARD_SIDE_LENGTH {
            panic!("Row out of bounds");
        }

        BoardIndex(row * BOARD_SIDE_LENGTH + col)
    }
}

// Learning: Implement From to convert a board index into a usize
impl From<BoardIndex> for usize {
    fn from(ix: BoardIndex) -> Self {
        ix.0
    }
}

// Learning: FromStr trait for supporting the parse method
impl FromStr for BoardIndex {
    type Err = &'static str;

    fn from_str(location: &str) -> Result<Self, Self::Err> {
        let location = location.as_bytes(); // Note shadowing

        // Check if length of location is ok (A1..C3).
        if location.len() != 2 {
            return Err("Invalid length");
        }

        // Parse column letter (A..C, a..c)
        let col = match location[0] {
            r @ b'A'..=b'C' => (r - b'A') as usize,
            r @ b'a'..=b'c' => (r - b'a') as usize,
            _ => return Err("Invalid column"),
        };

        // Parse the row digit (1..3)
        let row = match location[1] {
            c @ b'1'..=b'3' => (c - b'1') as usize,
            _ => return Err("Invalid row"),
        };

        Ok(BoardIndex::from_col_row(col, row))
    }
}
