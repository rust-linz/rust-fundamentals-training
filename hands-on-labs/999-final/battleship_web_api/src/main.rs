mod game_repository;

use std::str::FromStr;

use battleship_game_logic::BoardIndex;
use game_repository::{AddGameDto, GameRepository};
use rocket::fs::{FileServer, relative};
use rocket::{State, response::status::Created, serde::uuid::Uuid};
use rocket::serde::json::Json;
use rocket::serde::Serialize;

use crate::game_repository::ID;

#[macro_use] extern crate rocket;

#[post("/games", format = "json", data = "<game>")]
fn start_game(game: Json<AddGameDto<'_>>, games_state: &State<GameRepository>) -> Created<Json<Uuid>> {
    Created::new("")
        .body(Json(games_state.add(&game.0).id))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GetGameResponse {
    id: Uuid,
    player: String,
    board: String,
}

#[get("/games/<id>")]
fn get_game(id: ID, games_state: &State<GameRepository>) -> Option<Json<GetGameResponse>> {
    let game = games_state.get_by_id(&id)?;
    Some(Json(GetGameResponse { id: game.id, player: game.player.clone(), board: game.game.shooting_board_str() }))
}

#[derive(Responder)]
enum ShotResult {
    Success(Option<Json<GetGameResponse>>),
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
                Some(game) => ShotResult::Success(Some(Json(GetGameResponse { id: game.id, player: game.player.clone(), board: game.game.shooting_board_str() }))),
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
