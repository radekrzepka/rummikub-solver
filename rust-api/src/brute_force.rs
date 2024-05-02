use std::collections::HashSet;

use crate::game::Game;

const DEPTH: usize = 15;

pub fn find_best_game_brute_force(game: &Game) -> Option<Game> {
  println!("STARTING GAME:\n{}", game);

  let mut counter = 0;
  let mut all_games = HashSet::new();
  let best_game = find_best_game(game, DEPTH, &mut counter, &mut all_games);

  println!("At depth {}: Games calculated: {}", DEPTH, counter);

  if let Some(bg) = best_game {
    println!(
      "Best game found with {} tiles remaining:\n{}",
      bg.player_tiles.tiles.len(),
      bg
    );
    return Some(bg);
  }

  println!("No valid games found at this depth.");
  return None;
}

fn check_if_game_best(potential_best: &Game, best_game: &mut Option<Game>) {
  if best_game.is_none()
    || potential_best.player_tiles_length() < best_game.as_ref().unwrap().player_tiles_length()
  {
    *best_game = Some(potential_best.clone());
  }
}

fn find_best_game(
  current_game: &Game,
  depth: usize,
  counter: &mut usize,
  all_games: &mut HashSet<Game>,
) -> Option<Game> {
  *counter += 1;

  println!("{} {}", depth, counter);

  if !all_games.insert(current_game.clone()) {
    return None;
  }

  if depth == 0 {
    return Some(current_game.clone());
  }

  let legal_moves = current_game.get_legal_moves();
  let mut best_game: Option<Game> = None;

  for next_game in legal_moves {
    check_if_game_best(&next_game, &mut best_game);

    let potential_best = find_best_game(&next_game, depth - 1, counter, all_games);

    if let Some(potential_best) = potential_best {
      check_if_game_best(&potential_best, &mut best_game);
    }
  }

  return best_game;
}
