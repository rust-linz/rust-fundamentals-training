use std::ops::Index;

use crate::{BattleshipBoardContent, BoardFiller, BoardIndex, ShipFinder, ShipFindingResult, SquareContent, ToCompactString, random_placer};

#[derive(Debug, Copy, Clone)]
pub struct Shot {
    pub location: BoardIndex,
    pub result: SquareContent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum GameState {
    InProgress,
    AllShipsSunken,
    TooManyShots,
}

#[derive(Debug, Clone)]
pub struct SinglePlayerGame {
    log: Vec::<Shot>,
    board: BattleshipBoardContent,
    shooting_board: BattleshipBoardContent,
}

const SHIPS: &[usize] = &[5, 4, 3, 3, 2];

impl SinglePlayerGame {
    pub fn new() -> SinglePlayerGame {
        SinglePlayerGame::new_internal(true)
    }

    fn new_internal(fill: bool) -> SinglePlayerGame {
        let mut game = SinglePlayerGame {
            log: Vec::new(),
            board: BattleshipBoardContent::new_initialized(SquareContent::Water),
            shooting_board: BattleshipBoardContent::new(),
        };
        if fill { game.board.fill(SHIPS, random_placer); }
        game
    }

    pub fn board(&self) -> &impl Index<BoardIndex, Output = SquareContent> {
        &self.board
    }

    pub fn shooting_board(&self) -> &impl Index<BoardIndex, Output = SquareContent> {
        &self.shooting_board
    }

    pub fn shooting_board_str(&self) -> String {
        self.shooting_board.to_compact_str()
    }

    pub fn shoot(&mut self, ix: BoardIndex) -> SquareContent {
        let mut content;
        match self.shooting_board[ix] {
            SquareContent::Unknown => {
                // Player shoots on a square that she hasn't shot at before
                content = self.board[ix];
                self.shooting_board[ix] = content;
                if content == SquareContent::Ship {
                    // We have a hit
                    content = SquareContent::HitShip;
                    self.shooting_board[ix] = SquareContent::HitShip;

                    // Check whether the hit sank the ship
                    if let ShipFindingResult::CompleteShip(r) = self.board.try_find_ship(ix) {
                        if r.clone().into_iter().all(|i| matches!(self.shooting_board[i], SquareContent::HitShip | SquareContent::SunkenShip)) {
                            // The hit sank the ship -> change all ship quares to SunkenShip
                            content = SquareContent::SunkenShip;
                            for i in r {
                                self.shooting_board[i] = SquareContent::SunkenShip;
                            }
                        }
                    }

                }
            },
            _ => content = self.shooting_board[ix],
        };

        self.log.push(Shot { location: ix, result: content });
        content
    }

    pub fn game_state(&self) -> GameState {
        self.game_state_internal(SHIPS)
    }

    pub fn game_state_internal(&self, ships: &[usize]) -> GameState {
        if self.log.len() > 100 {
            return GameState::TooManyShots;
        }

        if self.shooting_board.iter().filter(|s| { matches!(s, SquareContent::HitShip | SquareContent::SunkenShip)}).count() == ships.iter().sum::<usize>() {
            return GameState::AllShipsSunken;
        }

        GameState::InProgress
    }

    pub fn log(&self) -> impl Iterator<Item = &Shot> {
        self.log.iter()
    }

    pub fn board_buffer(&self) -> *const SquareContent {
        self.shooting_board.board_buffer()
    }
}

impl Default for SinglePlayerGame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn clone() {
        let mut game = SinglePlayerGame::new_internal(false);
        game.shoot("A1".into());

        let clone = game.clone();
        assert_eq!(BoardIndex::from("A1"), clone.log[0].location);
    }

    #[test]
    fn shoot_into_water() {
        let mut game = SinglePlayerGame::new_internal(false);
        assert_eq!(SquareContent::Water, game.shoot("A1".into()));
        assert_eq!(SquareContent::Water, game.shooting_board[BoardIndex::from_str("A1").unwrap()]);
    }

    #[test]
    fn shoot_ship() {
        let mut game = SinglePlayerGame::new_internal(false);
        game.board[BoardIndex::from("A1")] = SquareContent::Ship;

        assert_eq!(SquareContent::HitShip, game.shoot("A1".into()));
        assert_eq!(SquareContent::HitShip, game.shooting_board[BoardIndex::from_str("A1").unwrap()]);
    }

    #[test]
    fn sink_ship() {
        let mut game = SinglePlayerGame::new_internal(false);
        game.board[BoardIndex::from("A1")] = SquareContent::Ship;
        game.board[BoardIndex::from("B1")] = SquareContent::Ship;

        assert_eq!(SquareContent::HitShip, game.shoot("A1".into()));
        assert_eq!(SquareContent::SunkenShip, game.shoot("B1".into()));
        assert_eq!(SquareContent::SunkenShip, game.shooting_board[BoardIndex::from("A1")]);
        assert_eq!(SquareContent::SunkenShip, game.shooting_board[BoardIndex::from("B1")]);
    }

    #[test]
    fn get_winner_in_progress() {
        let mut game = SinglePlayerGame::new_internal(false);
        game.board[BoardIndex::from("A1")] = SquareContent::HitShip;
        assert_eq!(GameState::InProgress, game.game_state());
       
    }

    #[test]
    fn get_winner_too_many_shots() {
        let mut game = SinglePlayerGame::new_internal(false);
        for _ in 0..=100 {
            assert_eq!(GameState::InProgress, game.game_state());
            game.shoot(BoardIndex::from(0));
        }

        assert_eq!(GameState::TooManyShots, game.game_state());
    }

    #[test]
    fn get_winner() {
        let mut game = SinglePlayerGame::new_internal(false);
        game.shooting_board[BoardIndex::from("A1")] = SquareContent::SunkenShip;
        assert_eq!(GameState::AllShipsSunken, game.game_state_internal(&[1]));
    }

    #[test]
    fn log() {
        let mut game = SinglePlayerGame::new_internal(false);
        game.shoot("A1".into());
        game.shoot("A2".into());

        assert_eq!(2, game.log().count());
        assert_eq!(BoardIndex::from_str("A1").unwrap(), game.log().nth(0).unwrap().location);
    }
}