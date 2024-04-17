use std::fs::File;

mod game;
mod tile;

use crate::game::{deserialize_game, Game};

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
      eprintln!("Failed to deserialize game: {}", e);
      return;
    }
  };

  println!("STARTING GAME:\n{}", game);
  check_game_integrity(&game);
}

fn check_game_integrity(game: &Game) {
  for depth in 1..=10 {
    let mut counter = 0;
    let best_game = find_best_game(game, depth, &mut counter, game.count_tiles());
    println!("At depth {}: Games calculated: {}", depth, counter);
    if let Some(bg) = best_game {
      println!(
        "Best game found with {} tiles remaining:\n{}",
        bg.player_tiles.tiles.len(),
        bg
      );
    } else {
      println!("No valid games found at this depth.");
    }
  }
}

fn find_best_game(
  current_game: &Game,
  depth: usize,
  counter: &mut usize,
  starting_tile_count: usize,
) -> Option<Game> {
  *counter += 1;

  if depth == 0 {
    return Some(current_game.clone());
  }

  let legal_moves = current_game.get_legal_moves();
  let mut best_game: Option<Game> = None;

  for next_game in legal_moves {
    let potential_best = find_best_game(&next_game, depth - 1, counter, starting_tile_count);
    if let Some(best) = potential_best {
      if best_game.is_none()
        || best.player_tiles.tiles.len() < best_game.as_ref().unwrap().player_tiles.tiles.len()
      {
        best_game = Some(best.clone());
      }
    }

    if next_game.count_tiles() != starting_tile_count {
      println!(
        "Tile count discrepancy detected at depth {}: Expected {}, Found {}",
        depth,
        starting_tile_count,
        next_game.count_tiles()
      );
      println!("Game state with discrepancy:\n{}", next_game);
    }
  }

  best_game
}
