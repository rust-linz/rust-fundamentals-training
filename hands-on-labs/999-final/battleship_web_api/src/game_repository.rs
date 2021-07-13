use std::{collections::HashMap, sync::RwLock};

use battleship_game_logic::{BoardIndex, SinglePlayerGame};
use uuid::Uuid;

/*
    Learnings in this module:

    * Mocking structs for unit tests
    * Conditional compilation

    Recommended readings for this module:

    * Mockall - mocking structs: https://docs.rs/mockall/0.10.1/mockall/#mocking-structs
    * Conditional compilation: https://doc.rust-lang.org/reference/conditional-compilation.html
*/

pub type ID = Uuid;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub player: String,
    pub game: SinglePlayerGame,
}

pub struct GameRepository {
    #[allow(dead_code)]
    games: RwLock<HashMap<ID, Game>>,
}

#[cfg(test)]
mockall::mock! {
    pub GameRepository {
        pub fn get_by_id(&self, id: &ID) -> Option<Game>;
        pub fn add(&self, player: String) -> Game;
        pub fn shoot(&self, id: &ID, location: BoardIndex) -> Option<Game>;
    }
}

impl Default for GameRepository {
    fn default() -> Self {
        GameRepository::new()
    }
}

impl GameRepository {
    pub fn new() -> GameRepository {
        GameRepository { games: RwLock::new(HashMap::new()) }
    }

    #[allow(dead_code)]
    pub fn add(&self, player: String) -> Game {
        let id = Uuid::new_v4();
        let new_game = Game { id, player, game: SinglePlayerGame::new() };

        let mut games = self.games.write().unwrap();
        games.insert(id, new_game.clone());
        new_game
    }

    #[allow(dead_code)]
    pub fn shoot(&self, id: &ID, location: BoardIndex) -> Option<Game> {
        let mut map = self.games.write().unwrap();
        let game = map.get_mut(id)?;

        game.game.shoot(location);
        Some(game.clone())
    }

    #[allow(dead_code)]
    pub fn get_by_id(&self, id: &ID) -> Option<Game> {
        let map = self.games.read().unwrap();
        let game = map.get(id)?;
        Some(game.clone())
    }
}
