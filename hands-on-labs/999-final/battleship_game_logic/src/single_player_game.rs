use std::ops::Index;

use crate::{BattleshipBoardContent, BoardFiller, BoardIndex, SquareContent, random_placer};

pub struct Shot {
    _location: BoardIndex,
    _result: SquareContent,
}

pub struct SinglePlayerGame {
    log: Vec::<Shot>,
    board: BattleshipBoardContent,
    shooting_board: BattleshipBoardContent,
}

impl SinglePlayerGame {
    pub fn new() -> SinglePlayerGame {
        let mut game = SinglePlayerGame {
            log: vec!(),
            board: BattleshipBoardContent::new_initialized(SquareContent::Water),
            shooting_board: BattleshipBoardContent::new(),
        };
        game.board.fill(&[5, 4, 3, 3, 2], random_placer);
        game
    }

    pub fn board(&self) -> &impl Index<BoardIndex, Output = SquareContent> {
        &self.board
    }

    pub fn shooting_board(&self) -> &impl Index<BoardIndex, Output = SquareContent> {
        &self.shooting_board
    }

    pub fn shoot(&mut self, ix: BoardIndex) -> SquareContent {
        let result = match self.shooting_board[ix] {
            SquareContent::Unknown => {
                let mut content = self.board[ix];
                self.shooting_board[ix] = content;
                if content == SquareContent::Ship {
                    content = SquareContent::HitShip;
                    self.shooting_board[ix] = SquareContent::HitShip;

                    // TODO: Complete

                }

                content
            },
            x => x,
        };
        self.log.push(Shot { _location: ix, _result: result });
        result
    }
}