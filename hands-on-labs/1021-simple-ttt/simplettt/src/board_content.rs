use std::{ops::{Index, IndexMut}, fmt};

use crate::board_index::BoardIndex;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SquareContent {
    X,
    O,
}

impl From<SquareContent> for char {
    fn from(value: SquareContent) -> Self {
        match value {
            SquareContent::X => 'X',
            SquareContent::O => 'O',
        }
    }
}

pub struct BoardContent {
    board_content: [Option<SquareContent>; 3 * 3],
}

impl BoardContent {
    pub fn new() -> Self {
        BoardContent {
            board_content: [None; 3 * 3],
        }
    }
}

// Learning: We can implement the Index trait to allow read-only indexing
impl Index<BoardIndex> for BoardContent {
    type Output = Option<SquareContent>;

    fn index(&self, ix: BoardIndex) -> &Self::Output {
        &self.board_content[usize::from(ix)]
    }
}

// Learning: We can implement the IndexMut trait to allow write indexing
impl IndexMut<BoardIndex> for BoardContent {
    fn index_mut(&mut self, ix: BoardIndex) -> &mut Self::Output {
        &mut self.board_content[usize::from(ix)]
    }
}

impl fmt::Display for BoardContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn build_separator(chars: &[char]) -> String {
            debug_assert!(chars.len() == 4);

            let mut result = String::new();
            result.reserve_exact(
                chars[0].len_utf8()
                    + 3 * 2 * chars[1].len_utf8()
                    + (3 - 1) * chars[2].len_utf8()
                    + chars[3].len_utf8()
                    + '\n'.len_utf8(),
            );
            result.push(chars[0]);
            for _ in 0..3 - 1 {
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
                + middle.len() * (3 - 1)
                + bottom.len()
                + (2 * '┃'.len_utf8()
                    + (3 - 1) * '|'.len_utf8()
                    + 3 * 2 * ' '.len_utf8()
                    + '\n'.len_utf8())
                    * 3,
        );

        result.push_str(&top);

        // Note: Here we use our rows iterator
        for row in 0..3 {
            result.push('┃');
            for col in 0..3 {
                let char = match self.board_content[row * 3 + col] { None => ' ', Some(x) => x.into() };
                result.push(char);
                result.push(char);
                if col < 2 {
                    result.push('|');
                }
            }

            result.push('┃');
            result.push('\n');
            if row < 2 {
                result.push_str(&middle);
            }
        }

        result.push_str(&bottom);

        write!(f, "{result}")
    }
}