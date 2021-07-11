use std::{collections::HashMap, sync::RwLock};
use rocket::serde::Deserialize;

use battleship_game_logic::{BoardIndex, SinglePlayerGame};
use uuid::Uuid;

pub type ID = Uuid;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddGameDto<'r> {
    player: &'r str,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub player: String,
    pub game: SinglePlayerGame,
}

pub struct GameRepository {
    games: RwLock<HashMap<ID, Game>>,
}

impl GameRepository {
    pub fn new() -> GameRepository {
        GameRepository { games: RwLock::new(HashMap::new()) }
    }

    pub fn add(&self, g: &AddGameDto) -> Game {
        let id = Uuid::new_v4();
        let new_game = Game { id, player: g.player.to_string(), game: SinglePlayerGame::new() };

        let mut games = self.games.write().unwrap();
        games.insert(id, new_game.clone());
        new_game
    }

    pub fn get_by_id(&self, id: &ID) -> Option<Game> {
        let map = self.games.read().unwrap();
        let game = map.get(id)?;
        Some(game.clone())
    }

    pub fn shoot(&self, id: &ID, location: BoardIndex) -> Option<Game> {
        let mut map = self.games.write().unwrap();
        let game = map.get_mut(id)?;

        game.game.shoot(location);
        Some(game.clone())
    }
}
