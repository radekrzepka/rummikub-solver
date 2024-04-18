use std::fs::File;

mod brute_force;
mod game;
mod tile;

use brute_force::find_best_game_brute_force;
use game::deserialize_game;

fn main() {
  for i in 8..=8 {
    println!("Game {}", i);

    let file_path = format!("testing-boards/board{}.json", i);
    let file = match File::open(&file_path) {
      Ok(file) => file,
      Err(e) => {
        eprintln!("Failed to open file '{}': {}", file_path, e);
        return;
      }
    };

    let game = match deserialize_game(file) {
      Ok(game) => game,
      Err(e) => {
        eprintln!("Failed to deserialize game: {}", e);
        return;
      }
    };

    find_best_game_brute_force(&game);
  }
}
