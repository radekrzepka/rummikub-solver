use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Color {
  Red,
  Blue,
  Yellow,
  Black,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum TileValue {
  Number(u8),
  Joker,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tile {
  pub value: TileValue,
  pub color: Color,
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.value {
      TileValue::Number(num) => write!(f, "{} {:?}", num, self.color),
      TileValue::Joker => write!(f, "Joker {:?}", self.color),
    }
  }
}

impl Tile {
  pub fn is_same_as(&self, other: &Tile) -> bool {
    self.value == other.value && self.color == other.color
  }
}

#[derive(Debug)]
pub enum WhereToAdd {
  Start,
  End,
}

#[derive(Debug)]
pub struct AddTileMove {
  pub tile: Tile,
  pub where_to_add: WhereToAdd,
  pub set_index: usize,
}

impl fmt::Display for AddTileMove {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Move: Add '{}' to the {} of set {}",
      self.tile,
      match self.where_to_add {
        WhereToAdd::Start => "start",
        WhereToAdd::End => "end",
      },
      self.set_index
    )
  }
}
