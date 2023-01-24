use crate::{board_content::{BoardContent, SquareContent}, board_index::BoardIndex};

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
        let current_value = self.content[ix];
        if current_value.is_some() {
            return Err("Square already set");
        }
        self.content[ix] = Some(self.current_player);

        self.current_player = match self.current_player {
            SquareContent::X => SquareContent::O,
            SquareContent::O => SquareContent::X,
        };

        Ok(())
    }

    #[allow(dead_code)]
    pub fn who_is_next(&self) -> SquareContent {
        self.current_player
    }
}


pub trait WinnerDetector {
    fn get_winner(&self) -> Option<SquareContent>;
}

impl WinnerDetector for Game
{
    /// Get the winner in the tic tac toe game; return None if no winner
    fn get_winner(&self) -> Option<SquareContent> {
        for row in 0..3 {
            if self.content[BoardIndex::from_col_row(0, row)].is_some()
                && self.content[BoardIndex::from_col_row(0, row)] == self.content[BoardIndex::from_col_row(1, row)]
                && self.content[BoardIndex::from_col_row(1, row)] == self.content[BoardIndex::from_col_row(2, row)]
            {
                return self.content[BoardIndex::from_col_row(1, row)];
            }
        }

        for col in 0..3 {
            if self.content[BoardIndex::from_col_row(col, 0)].is_some()
                && self.content[BoardIndex::from_col_row(col, 0)] == self.content[BoardIndex::from_col_row(col, 1)]
                && self.content[BoardIndex::from_col_row(col, 0)] == self.content[BoardIndex::from_col_row(col, 2)]
            {
                return self.content[BoardIndex::from_col_row(col, 0)];
            }
        }

        if self.content[BoardIndex::from_col_row(0, 0)].is_some()
            && self.content[BoardIndex::from_col_row(0, 0)] == self.content[BoardIndex::from_col_row(1, 1)]
            && self.content[BoardIndex::from_col_row(1, 1)] == self.content[BoardIndex::from_col_row(2, 2)] {
            return self.content[BoardIndex::from_col_row(0, 0)];
        }

        if self.content[BoardIndex::from_col_row(0, 2)].is_some()
            && self.content[BoardIndex::from_col_row(0, 2)] == self.content[BoardIndex::from_col_row(1, 1)]
            && self.content[BoardIndex::from_col_row(1, 1)] == self.content[BoardIndex::from_col_row(2, 0)] {
                return self.content[BoardIndex::from_col_row(0, 2)];
        }

        None
    }
}