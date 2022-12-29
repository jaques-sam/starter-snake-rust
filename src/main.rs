use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::instrument;

mod logic;

// API and Response Objects
// See https://docs.battlesnake.com/api

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,
    shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[instrument]
async fn handle_index() -> Json<Value> {
    Json(logic::info())
}

async fn handle_start(Json(start_req): Json<GameState>) -> impl IntoResponse {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    StatusCode::OK
}

async fn handle_move(Json(move_req): Json<GameState>) -> impl IntoResponse {
    let response = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    (StatusCode::OK, Json(response))
}

async fn handle_end(Json(end_req): Json<GameState>) {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Starting Battlesnake Server...");

    let app = Router::new()
        .route("/", get(handle_index))
        .route("/start", post(handle_start))
        .route("/move", post(handle_move))
        .route("/end", post(handle_end));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // TODO: convert to axum
    // rocket::build()
    // .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
    //     Box::pin(async move {
    //         res.set_raw_header("Server", "battlesnake/github/starter-snake-rust");
    //     })
    // }))
}
