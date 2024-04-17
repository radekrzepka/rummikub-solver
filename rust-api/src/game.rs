use std::{
  collections::{HashMap, HashSet},
  fmt,
  fs::File,
};

use serde::Deserialize;
use serde_json::Error;

use crate::tile::{Color, FromWhere, MoveType, Tile, TileMove, TileValue};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum SetType {
  Group,
  Run,
}

impl fmt::Display for SetType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      SetType::Group => write!(f, "Group:"),
      SetType::Run => write!(f, "Run:"),
    }
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Set {
  tiles: Vec<Tile>,
  set_type: SetType,
}

impl fmt::Display for Set {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} [", self.set_type)?;
    for (index, tile) in self.tiles.iter().enumerate() {
      if index > 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}", tile)?;
    }
    write!(f, "]")
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Board {
  sets: Vec<Set>,
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for set in &self.sets {
      write!(f, "{}\n", set)?;
    }
    Ok(())
  }
}

impl Board {
  fn handle_group(&self, set: &Set, set_index: usize) -> Vec<TileMove> {
    let mut moves = Vec::new();

    if set.tiles.len() != 3 {
      return moves;
    }

    let existing_colors: Vec<Color> = set.tiles.iter().map(|x| x.color.clone()).collect();
    let all_colors = vec![Color::Red, Color::Blue, Color::Yellow, Color::Black];

    for color in all_colors {
      if existing_colors.contains(&color) {
        continue;
      }

      if let TileValue::Number(num) = set.tiles[0].value {
        let new_move = TileMove {
          tile: Tile {
            value: TileValue::Number(num),
            color,
          },
          set_index,
          from_where: FromWhere::Start,
          move_type: MoveType::FromPlayerToBoard,
        };
        moves.push(new_move);
      }
    }

    return moves;
  }

  fn handle_run_start(&self, set: &Set, set_index: usize) -> Vec<TileMove> {
    let mut moves = Vec::new();

    if let Some(first_tile) = set.tiles.first() {
      if let TileValue::Number(first_num) = first_tile.value {
        if first_num > 1 {
          let new_move = TileMove {
            tile: Tile {
              value: TileValue::Number(first_num - 1),
              color: first_tile.color.clone(),
            },
            set_index,
            from_where: FromWhere::Start,
            move_type: MoveType::FromPlayerToBoard,
          };
          moves.push(new_move);
        }
      }
    }

    return moves;
  }

  fn handle_run_end(&self, set: &Set, set_index: usize) -> Vec<TileMove> {
    let mut moves = Vec::new();

    if let Some(last_tile) = set.tiles.last() {
      if let TileValue::Number(last_num) = last_tile.value {
        if last_num < 13 {
          let new_move = TileMove {
            tile: Tile {
              value: TileValue::Number(last_num + 1),
              color: last_tile.color.clone(),
            },
            set_index,
            from_where: FromWhere::End,
            move_type: MoveType::FromPlayerToBoard,
          };
          moves.push(new_move);
        }
      }
    }

    return moves;
  }

  fn tiles_to_add(&self) -> Vec<TileMove> {
    let mut new_moves: Vec<TileMove> = Vec::new();

    for (set_index, set) in self.sets.iter().enumerate() {
      match set.set_type {
        SetType::Group => new_moves.extend(self.handle_group(set, set_index)),
        SetType::Run => {
          new_moves.extend(self.handle_run_start(set, set_index));
          new_moves.extend(self.handle_run_end(set, set_index));
        }
      }
    }

    return new_moves;
  }

  fn tiles_to_take(&self) -> Vec<TileMove> {
    let mut removable_tiles = Vec::new();

    for (set_index, set) in self.sets.iter().enumerate() {
      match set.set_type {
        SetType::Run => {
          if set.tiles.len() <= 3 {
            continue;
          }

          if let Some(first_tile) = set.tiles.first() {
            let move_from_start = TileMove {
              tile: first_tile.clone(),
              from_where: FromWhere::Start,
              set_index,
              move_type: MoveType::FromBoardToPlayer,
            };
            removable_tiles.push(move_from_start);
          }

          if let Some(last_tile) = set.tiles.last() {
            let move_from_end = TileMove {
              tile: last_tile.clone(),
              from_where: FromWhere::End,
              set_index,
              move_type: MoveType::FromBoardToPlayer,
            };
            removable_tiles.push(move_from_end);
          }
        }
        SetType::Group => {
          if set.tiles.len() <= 3 {
            continue;
          }

          for tile in &set.tiles {
            let move_from_group = TileMove {
              tile: tile.clone(),
              from_where: FromWhere::Start,
              set_index,
              move_type: MoveType::FromBoardToPlayer,
            };
            removable_tiles.push(move_from_group);
          }
        }
      }
    }

    return removable_tiles;
  }

  fn split_runs(&self, player_tiles: &Vec<Tile>) -> Vec<(Board, Vec<Tile>)> {
    let mut new_configurations = Vec::new();

    for (set_index, set) in self.sets.iter().enumerate() {
      if set.set_type == SetType::Run && set.tiles.len() >= 6 {
        let splits = self.find_run_splits(&set.tiles);
        for split in splits {
          let mut new_sets = self.sets.clone();
          new_sets.remove(set_index);
          new_sets.extend(split);

          let new_board = Board { sets: new_sets };
          new_configurations.push((new_board, player_tiles.clone()));
        }
      }
    }

    new_configurations
  }

  fn find_run_splits(&self, tiles: &[Tile]) -> Vec<Vec<Set>> {
    let mut results = Vec::new();
    let length = tiles.len();

    for split_point in 3..(length - 2) {
      if split_point >= 3 && (length - split_point) >= 3 {
        let first_part = Set {
          tiles: tiles[0..split_point].to_vec(),
          set_type: SetType::Run,
        };
        let second_part = Set {
          tiles: tiles[split_point..].to_vec(),
          set_type: SetType::Run,
        };
        results.push(vec![first_part, second_part]);
      }
    }

    results
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Player {
  pub tiles: Vec<Tile>,
}

impl fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[")?;
    for (index, tile) in self.tiles.iter().enumerate() {
      if index > 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}", tile)?;
    }
    write!(f, "]\n")
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
  pub board: Board,
  pub player_tiles: Player,
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "\nBoard:\n{}Player Tiles: {}",
      self.board, self.player_tiles
    )
  }
}

impl Game {
  fn get_adding_tiles_games(&self) -> Vec<Game> {
    let mut legal_moves = Vec::new();
    let tiles_to_add = self.board.tiles_to_add();

    // for potential_move in &tiles_to_add {
    //   println!("Potential tile to add: {}", potential_move);
    // }
    // print!("\n");

    for tile_to_add in &tiles_to_add {
      let index = match self
        .player_tiles
        .tiles
        .iter()
        .position(|t| t == &tile_to_add.tile)
      {
        Some(i) => i,
        None => continue,
      };

      let mut new_player_tiles = self.player_tiles.tiles.clone();
      new_player_tiles.remove(index);

      let mut new_sets = self.board.sets.clone();
      match tile_to_add.from_where {
        FromWhere::Start => new_sets[tile_to_add.set_index]
          .tiles
          .insert(0, tile_to_add.tile.clone()),
        FromWhere::End => new_sets[tile_to_add.set_index]
          .tiles
          .push(tile_to_add.tile.clone()),
      }

      let new_board = Board { sets: new_sets };
      let new_player = Player {
        tiles: new_player_tiles,
      };

      legal_moves.push(Game {
        board: new_board,
        player_tiles: new_player,
      });
    }

    return legal_moves;
  }

  fn get_taking_tiles_games(&self) -> Vec<Game> {
    let tiles_to_move = self.board.tiles_to_take();
    let mut new_games = Vec::new();

    // for potential_move in &tiles_to_move {
    //   println!("Potential tile to delete: {}", potential_move);
    // }
    // print!("\n");

    for tile_move in tiles_to_move {
      let mut new_player_tiles = self.player_tiles.tiles.clone();
      new_player_tiles.push(tile_move.tile.clone());

      let mut new_sets = self.board.sets.clone();

      match tile_move.from_where {
        FromWhere::Start => Some(new_sets[tile_move.set_index].tiles.remove(0)),
        FromWhere::End => new_sets[tile_move.set_index].tiles.pop(),
      };

      let new_board = Board { sets: new_sets };
      let new_player = Player {
        tiles: new_player_tiles,
      };
      new_games.push(Game {
        board: new_board,
        player_tiles: new_player,
      });
    }

    return new_games;
  }

  fn create_new_sets_games(&self) -> Vec<Game> {
    let mut new_games = Vec::new();
    let possible_sets = self.find_possible_new_sets();

    for set in possible_sets {
      // println!("Potential set to create: {}", set);

      let mut new_sets = self.board.sets.clone();
      new_sets.push(set.clone());

      let new_board = Board { sets: new_sets };

      let mut new_player_tiles = self.player_tiles.tiles.clone();
      for tile in &set.tiles {
        if let Some(pos) = new_player_tiles.iter().position(|x| x == tile) {
          new_player_tiles.remove(pos);
        }
      }

      let new_player = Player {
        tiles: new_player_tiles,
      };

      new_games.push(Game {
        board: new_board,
        player_tiles: new_player,
      });
    }

    return new_games;
  }

  fn all_different_colors(tiles: &Vec<Tile>) -> bool {
    let mut color_set = HashSet::new();
    for tile in tiles {
      if !color_set.insert(tile.color.clone()) {
        return false;
      }
    }
    return true;
  }

  fn find_possible_new_sets(&self) -> Vec<Set> {
    let mut possible_sets = Vec::new();
    let grouped_by_number = self.group_tiles_by_number();
    let grouped_by_color = self.group_tiles_by_color();

    for (_, tiles) in &grouped_by_number {
      if tiles.len() >= 3 && Game::all_different_colors(tiles) {
        possible_sets.push(Set {
          tiles: tiles.clone(),
          set_type: SetType::Group,
        });
      }
    }
    for (_, tiles) in &grouped_by_color {
      let consecutive_tiles = self.find_consecutive_tiles(tiles);
      if consecutive_tiles.len() >= 3 {
        possible_sets.push(Set {
          tiles: consecutive_tiles,
          set_type: SetType::Run,
        });
      }
    }

    return possible_sets;
  }

  fn group_tiles_by_number(&self) -> HashMap<u8, Vec<Tile>> {
    let mut map = HashMap::new();

    for tile in &self.player_tiles.tiles {
      match tile.value {
        TileValue::Number(num) => {
          let entry = map.entry(num).or_insert_with(Vec::new);
          entry.push(tile.clone());
        }
        _ => {}
      }
    }

    return map;
  }

  fn group_tiles_by_color(&self) -> HashMap<Color, Vec<Tile>> {
    let mut map = HashMap::new();

    for tile in &self.player_tiles.tiles {
      let entry = map.entry(tile.color.clone()).or_insert_with(Vec::new);
      entry.push(tile.clone());
    }

    return map;
  }

  fn find_consecutive_tiles(&self, tiles: &[Tile]) -> Vec<Tile> {
    let mut sorted_tiles = tiles.to_vec();
    sorted_tiles.sort_by_key(|t| match t.value {
      TileValue::Number(num) => num,
      _ => 0,
    });

    let mut consecutive_tiles = Vec::new();
    let mut last_num = 0;

    for tile in sorted_tiles {
      match tile.value {
        TileValue::Number(num) if num == last_num + 1 || consecutive_tiles.is_empty() => {
          consecutive_tiles.push(tile.clone());
          last_num = num;
        }
        TileValue::Number(num) => {
          if consecutive_tiles.len() >= 3 {
            break;
          }
          consecutive_tiles.clear();
          consecutive_tiles.push(tile.clone());
          last_num = num;
        }
        _ => {}
      }
    }

    return consecutive_tiles;
  }

  fn split_games(&self) -> Vec<Game> {
    let mut new_games = Vec::new();

    let split_games = self.board.split_runs(&self.player_tiles.tiles);
    for (new_board, new_player_tiles) in split_games {
      new_games.push(Game {
        board: new_board,
        player_tiles: Player {
          tiles: new_player_tiles,
        },
      });
    }

    return new_games;
  }

  pub fn count_tiles(&self) -> usize {
    self.board.sets.iter().flat_map(|s| &s.tiles).count() + self.player_tiles.tiles.len()
  }

  pub fn get_legal_moves(&self) -> Vec<Game> {
    let mut legal_moves = Vec::new();

    legal_moves.extend(self.get_adding_tiles_games()); //1. Get all possible moves player -> board
    legal_moves.extend(self.get_taking_tiles_games()); //2. Get all possible moves board -> player
    legal_moves.extend(self.create_new_sets_games()); //3. Create new sets
    legal_moves.extend(self.split_games()); //4. Split sets

    return legal_moves;
  }
}

pub fn deserialize_game(file: File) -> Result<Game, serde_json::Error> {
  let game: Result<Game, Error> = serde_json::from_reader(file);

  return game;
}
