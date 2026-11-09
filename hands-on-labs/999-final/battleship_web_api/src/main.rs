mod game_repository;

use std::{str::FromStr, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use battleship_game_logic::BoardIndex;
use game_repository::{Game, ID};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[cfg_attr(test, mockall_double::double)]
use game_repository::GameRepository;

/*
    Learnings in this module:

    * Building a simple web API with Axum
    * Extractors and shared state in async handlers
    * Custom error responses with `IntoResponse`
    * Testing web APIs
    * Mocking structs with doubles

    Recommended readings for this module:

    * Axum docs: https://docs.rs/axum/latest/axum/
    * Sharing state with handlers: https://docs.rs/axum/latest/axum/#sharing-state-with-handlers
    * Error handling: https://docs.rs/axum/latest/axum/error_handling/index.html
    * `mockall_double` crate: https://docs.rs/mockall_double/latest/mockall_double/
*/

// Note: The state has to be cloneable because Axum clones it for every request.
//       `Arc` makes that cheap and lets all requests share the same repository.
type Games = State<Arc<GameRepository>>;

#[derive(Deserialize)]
struct AddGameDto {
    player: String,
}

// Note: The extractor consuming the request body (`Json`) has to be the last parameter.
async fn start_game(State(games): Games, Json(game): Json<AddGameDto>) -> (StatusCode, Json<ID>) {
    (StatusCode::CREATED, Json(games.add(game.player).id))
}

#[derive(Serialize)]
struct GameResponse {
    id: ID,
    player: String,
    board: String,
    game_status: u8,
}

impl GameResponse {
    fn from_game(game: &Game) -> GameResponse {
        GameResponse {
            id: game.id,
            player: game.player.clone(),
            board: game.game.shooting_board_str(),
            game_status: game.game.game_state() as u8,
        }
    }
}

async fn get_game(Path(id): Path<ID>, State(games): Games) -> Result<Json<GameResponse>, StatusCode> {
    let game = games.get_by_id(&id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(GameResponse::from_game(&game)))
}

enum ShotError {
    GameNotFound,
    InvalidLocation,
}

impl IntoResponse for ShotError {
    fn into_response(self) -> Response {
        match self {
            ShotError::GameNotFound => (StatusCode::NOT_FOUND, "No game found with given id"),
            ShotError::InvalidLocation => (StatusCode::BAD_REQUEST, "Invalid board index"),
        }
        .into_response()
    }
}

async fn shoot(
    Path(id): Path<ID>,
    State(games): Games,
    Json(location): Json<String>,
) -> Result<Json<GameResponse>, ShotError> {
    let location = BoardIndex::from_str(&location).map_err(|_| ShotError::InvalidLocation)?;
    let game = games.shoot(&id, location).ok_or(ShotError::GameNotFound)?;
    Ok(Json(GameResponse::from_game(&game)))
}

fn app(games: GameRepository) -> Router {
    Router::new()
        .route("/games", post(start_game))
        .route("/games/{id}", get(get_game))
        .route("/games/{id}/shoot", post(shoot))
        .with_state(Arc::new(games))
        .fallback_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app(GameRepository::new())).await.unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_repository::MockGameRepository;
    use axum::{
        body::{Body, Bytes},
        http::{Request, header},
    };
    use battleship_game_logic::SinglePlayerGame;
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt; // Provides `oneshot`
    use uuid::Uuid;

    const GAME_ID: &str = "822c594e-b5ee-4ca5-ae3b-86d8bb97b43a";

    fn game() -> Game {
        Game {
            id: Uuid::from_str(GAME_ID).unwrap(),
            player: "FooBar".to_string(),
            game: SinglePlayerGame::new(),
        }
    }

    fn post_json(uri: &str, body: Value) -> Request<Body> {
        Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    fn get_request() -> Request<Body> {
        Request::get(format!("/games/{GAME_ID}")).body(Body::empty()).unwrap()
    }

    // Note: `oneshot` sends a single request directly to the router, no TCP involved.
    async fn send(repo: MockGameRepository, request: Request<Body>) -> (StatusCode, Bytes) {
        let response = app(repo).oneshot(request).await.unwrap();
        let status = response.status();
        (status, response.into_body().collect().await.unwrap().to_bytes())
    }

    #[tokio::test]
    async fn add() {
        let mut repo = MockGameRepository::default();
        repo.expect_add().return_const(game());

        let (status, body) = send(repo, post_json("/games", json!({ "player": "FooBar" }))).await;

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(serde_json::from_slice::<Value>(&body).unwrap(), json!(GAME_ID));
    }

    #[tokio::test]
    async fn get_success() {
        let mut repo = MockGameRepository::default();
        repo.expect_get_by_id().return_const(game());

        let (status, body) = send(repo, get_request()).await;

        assert_eq!(status, StatusCode::OK);
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["player"], "FooBar");
        assert_eq!(body["board"], " ".repeat(100)); // Ship positions must stay secret
    }

    #[tokio::test]
    async fn get_not_found() {
        let mut repo = MockGameRepository::default();
        repo.expect_get_by_id().return_const(None);

        let (status, _) = send(repo, get_request()).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn shoot_success() {
        let mut repo = MockGameRepository::default();
        repo.expect_shoot().return_const(game());

        let (status, _) = send(repo, post_json(&format!("/games/{GAME_ID}/shoot"), json!("B1"))).await;

        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn shoot_game_not_found() {
        let mut repo = MockGameRepository::default();
        repo.expect_shoot().return_const(None);

        let (status, _) = send(repo, post_json(&format!("/games/{GAME_ID}/shoot"), json!("B1"))).await;

        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn shoot_invalid_location() {
        let repo = MockGameRepository::default();

        let (status, _) = send(repo, post_json(&format!("/games/{GAME_ID}/shoot"), json!("Z99"))).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
