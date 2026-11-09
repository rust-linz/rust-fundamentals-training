use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{
    board_content::{BOARD_SIDE_LENGTH, BOARD_SQUARE_COUNT},
    board_index::BoardIndex,
    square_content::SquareContent,
};

/// All lines that win the game (rows, columns, diagonals) as board indexes
const WINNING_LINES: [[usize; BOARD_SIDE_LENGTH]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

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
    SquareAlreadyHasValue,
    WrongPlayerSetOrder,
}

impl Display for SetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.error_kind {
            ErrorKind::SquareAlreadyHasValue => "Square is already taken",
            ErrorKind::WrongPlayerSetOrder => "Players have to take turns",
        })
    }
}

impl std::error::Error for SetError {}

#[cfg_attr(test, mockall::automock)]
pub trait SquareAccessor {
    fn get(&self, ix: BoardIndex) -> Option<SquareContent>;
    fn set(&mut self, ix: BoardIndex, value: Option<SquareContent>);
}

impl<T> SquareAccessor for T
where
    T: Index<BoardIndex, Output = Option<SquareContent>>
        + IndexMut<BoardIndex, Output = Option<SquareContent>>,
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
        let sum = (0..BOARD_SQUARE_COUNT)
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

    #[allow(dead_code)]
    pub fn who_is_next(&self) -> SquareContent {
        self.current_player
    }

    /// Returns true if no empty square is left
    pub fn is_board_full(&self) -> bool {
        (0..BOARD_SQUARE_COUNT).all(|ix| self.content.get(BoardIndex::from_index(ix)).is_some())
    }
}

pub trait WinnerDetector {
    fn get_winner(&self) -> Option<SquareContent>;
}

impl<T: SquareAccessor> WinnerDetector for Game<T> {
    /// Get the winner in the tic tac toe game; return None if no winner
    fn get_winner(&self) -> Option<SquareContent> {
        WINNING_LINES.iter().find_map(|line| {
            let [a, b, c] = line.map(|ix| self.content.get(BoardIndex::from_index(ix)));

            // Note: If all squares are empty, `a` is None and the search continues
            if a == b && b == c { a } else { None }
        })
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
    use crate::board_content::BoardContent;
    use mockall::predicate::eq;
    use rstest::rstest;

    #[rstest]
    #[case([0, 1, 2])]
    #[case([3, 4, 5])]
    #[case([6, 7, 8])]
    #[case([0, 3, 6])]
    #[case([1, 4, 7])]
    #[case([2, 5, 8])]
    #[case([0, 4, 8])]
    #[case([2, 4, 6])]
    fn winner(#[case] line: [usize; BOARD_SIDE_LENGTH]) {
        let mut board = BoardContent::new();
        for ix in line {
            board[ix] = Some(SquareContent::O);
        }

        assert_eq!(Some(SquareContent::O), Game::new(board).get_winner());
    }

    #[test]
    fn no_winner() {
        let mut board = BoardContent::new();
        board[0] = Some(SquareContent::X);
        board[1] = Some(SquareContent::O);
        board[2] = Some(SquareContent::X);

        assert_eq!(None, Game::new(board).get_winner());
    }

    #[test]
    fn set_occupied_square_fails() {
        let mut game = Game::new(BoardContent::new());
        assert!(game.set(BoardIndex::from_index(0)).is_ok());

        let expected = SetError {
            error_kind: ErrorKind::SquareAlreadyHasValue,
        };
        assert_eq!(Err(expected), game.set(BoardIndex::from_index(0)));

        // The failed attempt must not change whose turn it is
        assert_eq!(SquareContent::O, game.who_is_next());
    }

    #[test]
    fn board_full() {
        let mut game = Game::new(BoardContent::new());
        for ix in 0..BOARD_SQUARE_COUNT {
            assert!(!game.is_board_full());
            game.set(BoardIndex::from_index(ix)).unwrap();
        }

        assert!(game.is_board_full());
    }

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
        assert!(game.set(BoardIndex::from_index(0)).is_ok());
    }
}
