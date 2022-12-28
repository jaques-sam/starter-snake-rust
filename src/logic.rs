// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Game, Coord};

mod moves;
use moves::{Direction};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// Opening the Battlesnake URL in a browser shows this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "DrumSn8ke",
        "color": "#508c50",
        "head": "earmuffs",
        "tail": "nr-booster",
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let mut safe_moves: HashMap<_, _> = vec![
        (Direction::Up, true),
        (Direction::Down, true),
        (Direction::Left, true),
        (Direction::Right, true),
    ]
    .into_iter()
    .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"

    if my_neck.x < my_head.x {
        // Neck is left of head, don't move left
        safe_moves.insert(Direction::Left, false);
    } else if my_neck.x > my_head.x {
        // Neck is right of head, don't move right
        safe_moves.insert(Direction::Right, false);
    } else if my_neck.y < my_head.y {
        // Neck is below head, don't move down
        safe_moves.insert(Direction::Down, false);
    } else if my_neck.y > my_head.y {
        // Neck is above head, don't move up
        safe_moves.insert(Direction::Up, false);
    }

    // Prevent your Battlesnake from moving out of bounds
    let board_width = &board.width;
    let board_height = &board.height;
    if my_head.x >= board_width - 1 {
        safe_moves.insert(Direction::Right, false);
    }
    if my_head.y >= board_height -1 {
        safe_moves.insert(Direction::Up, false);
    }
    if my_head.x <= 0 {
        safe_moves.insert(Direction::Left, false);
    }
    if my_head.y <= 0 {
        safe_moves.insert(Direction::Down, false);
    }

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    // let my_body = &you.body;

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    let safe_moves = safe_moves
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    
    // Choose a random move from the safe ones
    let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen.to_string() });
}
