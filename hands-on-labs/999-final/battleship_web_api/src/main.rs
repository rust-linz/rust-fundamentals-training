mod game_repository;

use std::str::FromStr;
use battleship_game_logic::BoardIndex;
use game_repository::Game;
use game_repository::ID;

#[cfg_attr(test, mockall_double::double)]
use game_repository::GameRepository;
use rocket::fs::{FileServer, relative};
use rocket::{State, response::status::Created, serde::uuid::Uuid};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

/*
    Learnings in this module:

    * Building a simple web API with Rocket v0.5-rc
    * Testing web APIs
    * Mocking structs with doubles

    Recommended readings for this module:

    * Rocket docs: https://rocket.rs/v0.5-rc/guide/introduction/
    * `mockall_double` crate: https://docs.rs/mockall_double/0.2.0/mockall_double/
*/

#[macro_use] extern crate rocket;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddGameDto {
    player: String,
}

#[post("/games", format = "json", data = "<game>")]
fn start_game(game: Json<AddGameDto>, games_state: &State<GameRepository>) -> Created<Json<Uuid>> {
    Created::new("")
        .body(Json(games_state.add(game.0.player).id))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GameResponse {
    id: Uuid,
    player: String,
    board: String,
    game_status: u8
}

impl GameResponse {
    fn from_game(game: &Game) -> GameResponse {
        GameResponse {
            id: game.id,
            player:
            game.player.clone(),
            board: game.game.shooting_board_str(),
            game_status: game.game.game_state() as u8,
        }
    }
}

#[get("/games/<id>")]
fn get_game(id: ID, games_state: &State<GameRepository>) -> Option<Json<GameResponse>> {
    let game = games_state.get_by_id(&id)?;
    Some(Json(GameResponse::from_game(&game)))
}

#[derive(Responder)]
enum ShotResult {
    Success(Option<Json<GameResponse>>),
    #[response(status = 404)]
    NotFound(&'static str),
    #[response(status = 400)]
    BadRequest(&'static str)
}

#[post("/games/<id>/shoot", format = "json", data = "<location>")]
fn shoot(id: ID, location: Json<&'_ str>, games_state: &State<GameRepository>) -> ShotResult {
    match BoardIndex::from_str(location.as_ref()) {
        Ok(location) => {
            match games_state.shoot(&id, location) {
                Some(game) => ShotResult::Success(Some(Json(GameResponse::from_game(&game)))),
                None => ShotResult::NotFound("No game found with given id"),
            }
            
        },
        Err(_) => ShotResult::BadRequest("Invalid board index")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage( GameRepository::new())
        .mount("/", routes![start_game,  get_game, shoot])
        .mount("/", FileServer::from(relative!("public")))
}

#[cfg(test)]
mod test {
    use crate::{AddGameDto, game_repository::{Game, MockGameRepository}, get_game, start_game};
    
    use super::rocket;
    use battleship_game_logic::SinglePlayerGame;
    use rocket::{http::Status, local::blocking::Client};
    use uuid::Uuid;
    
    #[test]
    fn add() {
        let mut repo: MockGameRepository = MockGameRepository::default();
        repo.expect_add().return_const(Game { id: Uuid::new_v4(), player: "FooBar".to_string(), game: SinglePlayerGame::new() });

        let r = rocket::build()
            .manage( repo)
            .mount("/", routes![start_game]);

        let client = Client::tracked(r).unwrap();
        let response = client.post("/games").json(&AddGameDto { player: "FooBar".into() }).dispatch();
        assert_eq!(response.status(), Status::Created);
    }
    
    #[test]
    fn get_success() {
        let mut repo: MockGameRepository = MockGameRepository::default();
        repo.expect_get_by_id().return_const(Game { id: Uuid::new_v4(), player: "FooBar".to_string(), game: SinglePlayerGame::new() });

        let r = rocket::build()
            .manage( repo)
            .mount("/", routes![get_game]);

        let client = Client::tracked(r).unwrap();
        let response = client.get("/games/822c594e-b5ee-4ca5-ae3b-86d8bb97b43a").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    
    #[test]
    fn get_not_found() {
        let mut repo: MockGameRepository = MockGameRepository::default();
        repo.expect_get_by_id().return_const(None);

        let r = rocket::build()
            .manage( repo)
            .mount("/", routes![get_game]);

        let client = Client::tracked(r).unwrap();
        let response = client.get("/games/822c594e-b5ee-4ca5-ae3b-86d8bb97b43a").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}