use std::fs::File;

use serde_json::Error;
use serde::Deserialize;

use crate::tile::Tile;

#[derive(Debug,Deserialize)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
}
#[derive(Debug,Deserialize)]
pub struct Player {
    tiles: Vec<Tile>,
}
#[derive(Debug,Deserialize)]
pub struct Game {
    board: Board,
    player_tiles: Player,
}

pub fn deserialize_game (file: File) -> Result<Vec<Game>, serde_json::Error> { 
    let games: Result<Vec<Game>, Error> = serde_json::from_reader(file);

    return games;
} 
