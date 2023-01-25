use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct BoardIndex(usize);

impl BoardIndex {
    pub fn new() -> BoardIndex {
        BoardIndex(0)
    }

    pub fn from_col_row(col: usize, row: usize) -> BoardIndex {
        if col >= 3 {
            panic!("Column out of bounds");
        }

        if row >= 3 {
            panic!("Row out of bounds");
        }

        BoardIndex(row * 3 + col)
    }

    pub fn column(&self) -> usize {
        self.0 % 3
    }

    pub fn row(&self) -> usize {
        self.0 / 3
    }
}

impl Default for BoardIndex {
    fn default() -> Self {
        Self::new()
    }
}

// Note: Some type casting traits, similar to what we did in square_content
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

        // Parse the row letter(s) (1..3)
        let row = match location[1] {
            c @ b'1'..=b'3' => (c - b'1') as usize,
            _ => return Err("Invalid row"),
        };

        Ok(BoardIndex::from_col_row(col, row))
    }
}
