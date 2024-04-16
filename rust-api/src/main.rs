use std::fs::File;

use crate::game::deserialize_game;

mod game;
mod tile;

fn main() {
  let file_path = "src/board.json";
  let file = match File::open(file_path) {
    Ok(file) => file,
    Err(e) => {
      eprintln!("Failed to open file '{}': {}", file_path, e);
      return;
    }
  };

  let game = match deserialize_game(file) {
    Ok(game) => game,
    Err(e) => {
      eprintln!("Failed to deserialize games: {}", e);
      return;
    }
  };

  println!("STARTING {}", game);
  let legal_moves = game.get_legal_moves();

  for (i, game) in legal_moves.into_iter().enumerate() {
    println!("{} {}", i, game);
  }
}
