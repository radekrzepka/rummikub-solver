use std::fs::{self, File};

use game::Game;

use crate::game::deserialize_game;

mod game;
mod tile;
mod solution;

fn print_data (games: &Vec<Game>)  {
    for (index, game) in games.into_iter().enumerate() {
        println!("{}: {:?}", index, game);
    }
}
fn main() {
    let file_path = "src/boards.json";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file '{}': {}", file_path, e);
            return;
        },
    };

    let games = match deserialize_game(file) {
        Ok(games) => games,
        Err(e) => {
            eprintln!("Failed to deserialize games: {}", e);
            return;
        },
    };

    print_data(&games);
    println!("{:?}", games)
}
