use crate::{
    board_content::{BOARD_SIDE_LENGTH, BoardContent, SquareContent},
    board_index::BoardIndex,
};

/// All lines that win the game (rows, columns, diagonals) as (column, row) coordinates
const WINNING_LINES: [[(usize, usize); BOARD_SIDE_LENGTH]; 8] = [
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(2, 0), (1, 1), (0, 2)],
];

pub struct Game {
    pub content: BoardContent,
    current_player: SquareContent,
}

impl Game {
    pub fn new(content: BoardContent) -> Self {
        Game {
            content,
            current_player: SquareContent::X,
        }
    }

    pub fn set(&mut self, ix: BoardIndex) -> Result<(), &'static str> {
        if self.content[ix].is_some() {
            return Err("Square already set");
        }
        self.content[ix] = Some(self.current_player);

        self.current_player = match self.current_player {
            SquareContent::X => SquareContent::O,
            SquareContent::O => SquareContent::X,
        };

        Ok(())
    }

    /// Get the winner in the tic tac toe game; return None if no winner
    pub fn get_winner(&self) -> Option<SquareContent> {
        WINNING_LINES.iter().find_map(|line| {
            let [a, b, c] = line.map(|(col, row)| self.content[BoardIndex::from_col_row(col, row)]);

            // Note: If all squares are empty, `a` is None and the search continues
            if a == b && b == c { a } else { None }
        })
    }
}
