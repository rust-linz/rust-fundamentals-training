mod utils;

use std::str::FromStr;

use battleship_game_logic::{BoardIndex, GameState, SinglePlayerGame, SquareContent};
use wasm_bindgen::prelude::*;
use serde::Serialize;

/*
    Learnings in this module:

    * Interactions between WASM modules and JavaScript
    * `wasm_bindgen`
    * wasm-pack tools

    Recommended readings for this module:

    * Rust WASM book: https://rustwasm.github.io/docs/book/introduction.html
    * The `wasm-bindgen` Guide: https://rustwasm.github.io/wasm-bindgen/introduction.html
    * wasm-pack docs: https://rustwasm.github.io/docs/wasm-pack/
*/

#[wasm_bindgen]
pub struct BattleshipGame {
    game: SinglePlayerGame,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Serialize)]
pub enum SquareContentJS {
    Water = 0,
    HitShip = 2,
    SunkenShip = 3,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Serialize)]
pub enum GameStateJS {
    InProgress,
    AllShipsSunken,
    TooManyShots,
}

#[wasm_bindgen]
impl BattleshipGame {
    pub fn new() -> BattleshipGame {
        BattleshipGame { game: SinglePlayerGame::new() }
    }

    pub fn board_content(&self) -> *const SquareContent {
        self.game.board_buffer()
    }

    pub fn shoot(&mut self, location: &str) -> Result<JsValue, JsValue> {
        match BoardIndex::from_str(location) {
            Ok(location) => {
                #[derive(Serialize)]
                struct Result {
                    shot_result: SquareContentJS,
                    game_state: GameStateJS,
                }

                let shot_result = self.game.shoot(location);
                Ok(JsValue::from_serde(&Result {
                    shot_result: match shot_result {
                        SquareContent::Water => SquareContentJS::Water,
                        SquareContent::HitShip => SquareContentJS::HitShip,
                        SquareContent::SunkenShip => SquareContentJS::SunkenShip,
                        _ => panic!("Invalid square content"),
                    },
                    game_state: match self.game.game_state() {
                        GameState::InProgress => GameStateJS::InProgress,
                        GameState::AllShipsSunken => GameStateJS::AllShipsSunken,
                        GameState::TooManyShots => GameStateJS::TooManyShots
                    }
                }).unwrap())
            },
            Err(e) => Err(e.into())
        }
    }
}

impl Default for BattleshipGame {
    fn default() -> Self {
        BattleshipGame::new()
    }
}