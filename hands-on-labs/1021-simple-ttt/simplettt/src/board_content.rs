use std::{
    fmt,
    ops::{Index, IndexMut},
};

use crate::board_index::BoardIndex;

/// Number of rows and columns of the board
pub const BOARD_SIDE_LENGTH: usize = 3;

/// Total number of squares on the board
pub const BOARD_SQUARE_COUNT: usize = BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH;

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
    board_content: [Option<SquareContent>; BOARD_SQUARE_COUNT],
}

impl BoardContent {
    pub fn new() -> Self {
        BoardContent {
            board_content: [None; BOARD_SQUARE_COUNT],
        }
    }

    /// Returns true if no empty square is left
    pub fn is_full(&self) -> bool {
        self.board_content.iter().all(Option::is_some)
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
                    + BOARD_SIDE_LENGTH * 2 * chars[1].len_utf8()
                    + (BOARD_SIDE_LENGTH - 1) * chars[2].len_utf8()
                    + chars[3].len_utf8()
                    + '\n'.len_utf8(),
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

        for row in 0..BOARD_SIDE_LENGTH {
            result.push('┃');
            for col in 0..BOARD_SIDE_LENGTH {
                let char = match self.board_content[row * BOARD_SIDE_LENGTH + col] {
                    None => ' ',
                    Some(x) => x.into(),
                };
                result.push(char);
                result.push(char);
                if col < BOARD_SIDE_LENGTH - 1 {
                    result.push('|');
                }
            }

            result.push('┃');
            result.push('\n');
            if row < BOARD_SIDE_LENGTH - 1 {
                result.push_str(&middle);
            }
        }

        result.push_str(&bottom);

        write!(f, "{result}")
    }
}
