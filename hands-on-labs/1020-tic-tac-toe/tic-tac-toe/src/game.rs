use std::{ops::{Index, IndexMut}, fmt::Display};

use crate::{board_index::BoardIndex, square_content::SquareContent, row::Rows};

pub struct Game<T> {
    content: T,
    current_player: SquareContent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SetError {
    error_kind: ErrorKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    CannotResetToEmpty,
    SquareAlreadyHasValue,
    WrongPlayerSetOrder,
}

#[cfg_attr(test, mockall::automock)]
pub trait SquareAccessor {
    fn get(&self, ix: BoardIndex) -> Option<SquareContent>;
    fn set(&mut self, ix: BoardIndex, value: Option<SquareContent>);
}

impl<T> SquareAccessor for T
where
    T: Index<BoardIndex, Output = Option<SquareContent>> + IndexMut<BoardIndex, Output = Option<SquareContent>>,
{
    fn get(&self, ix: BoardIndex) -> Option<SquareContent> {
        self[ix]
    }

    fn set(&mut self, ix: BoardIndex, value: Option<SquareContent>) {
        self[ix] = value;
    }
}

impl<T: SquareAccessor> Game<T> {
    pub fn new(content: T) -> Self {
        Game {
            content,
            current_player: SquareContent::X,
        }
    }

    fn is_valid(&self) -> bool {
        let sum = (0..9)
            .map(|ix| match self.content.get(BoardIndex::from_index(ix)) {
                None => 0,
                Some(SquareContent::X) => 1,
                Some(SquareContent::O) => -1,
            })
            .sum();
        matches!(sum, -1..=1)
    }

    pub fn set(&mut self, ix: BoardIndex) -> Result<(), SetError> {
        let current_value = self.content.get(ix);
        if current_value.is_some() {
            return Err(SetError {
                error_kind: ErrorKind::SquareAlreadyHasValue,
            });
        }
        self.content.set(ix, Some(self.current_player));
        if !self.is_valid() {
            self.content.set(ix, current_value);
            return Err(SetError {
                error_kind: ErrorKind::WrongPlayerSetOrder,
            });
        }
        self.current_player = match self.current_player {
            SquareContent::X => SquareContent::O,
            SquareContent::O => SquareContent::X,
        };

        Ok(())
    }

    pub fn who_is_next(&self) -> SquareContent {
        self.current_player
    }
}

pub trait WinnerDetector {
    fn get_winner(&self) -> Option<SquareContent>;
}

impl<T> WinnerDetector for Game<T>
where
    T: SquareAccessor + Rows,
{
    /// Get the winner in the tic tac toe game; return None if no winner
    fn get_winner(&self) -> Option<SquareContent> {
        for row in self.content.rows() {
            if row[0].is_some()
                && row[0] == row[1]
                && row[1] == row[2]
            {
                return row[0];
            }
        }

        for col in 0..3 {
            if self.content.get(BoardIndex::from_col_row(col, 0)).is_some()
                && self.content.get(BoardIndex::from_col_row(col, 0)) == self.content.get(BoardIndex::from_col_row(col, 1))
                && self.content.get(BoardIndex::from_col_row(col, 0)) == self.content.get(BoardIndex::from_col_row(col, 2))
            {
                return self.content.get(BoardIndex::from_col_row(col, 0));
            }
        }

        if self.content.get(BoardIndex::from_col_row(0, 0)).is_some()
            && self.content.get(BoardIndex::from_col_row(0, 0)) == self.content.get(BoardIndex::from_col_row(1, 1))
            && self.content.get(BoardIndex::from_col_row(1, 1)) == self.content.get(BoardIndex::from_col_row(2, 2)) {
            return self.content.get(BoardIndex::from_col_row(0, 0));
        }

        if self.content.get(BoardIndex::from_col_row(0, 2)).is_some()
            && self.content.get(BoardIndex::from_col_row(0, 2)) == self.content.get(BoardIndex::from_col_row(1, 1))
            && self.content.get(BoardIndex::from_col_row(1, 1)) == self.content.get(BoardIndex::from_col_row(2, 0)) {
                return self.content.get(BoardIndex::from_col_row(0, 2));
        }

        None
    }
}

impl<T: Display> Display for Game<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use rstest::rstest;

    #[test]
    fn is_valid() {
        let mut mock = MockSquareAccessor::new();
        mock.expect_get()
            .returning(|ix: BoardIndex| match Into::<usize>::into(ix) % 2 {
                0 => Some(SquareContent::X),
                1 => Some(SquareContent::O),
                _ => panic!("Must never happen"),
            });

        let game = Game {
            content: mock,
            current_player: SquareContent::X,
        };
        assert!(game.is_valid());
    }

    #[rstest]
    #[case(SquareContent::X)]
    #[case(SquareContent::O)]
    fn is_not_valid(#[case] value: SquareContent) {
        let mut mock = MockSquareAccessor::new();
        mock.expect_get()
            .returning(move |ix: BoardIndex| match Into::<usize>::into(ix) {
                0..=1 => Some(value),
                _ => None,
            });

        let game = Game {
            content: mock,
            current_player: SquareContent::X,
        };
        assert!(!game.is_valid());
    }

    #[test]
    fn set() {
        let mut mock = MockSquareAccessor::new();
        mock.expect_get().return_const(None);
        mock.expect_set()
            .with(eq(BoardIndex::from_index(0)), eq(Some(SquareContent::X)))
            .return_const(())
            .times(1);
        let mut game = Game {
            content: mock,
            current_player: SquareContent::X,
        };
        assert!(game.set(0.into()).is_ok());
    }
}
