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
pub enum FromWhere {
  Start,
  End,
}

#[derive(Debug)]
pub enum MoveType {
  FromPlayerToBoard,
  FromBoardToPlayer,
}

#[derive(Debug)]
pub struct TileMove {
  pub tile: Tile,
  pub from_where: FromWhere,
  pub set_index: usize,
  pub move_type: MoveType,
}

impl fmt::Display for TileMove {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Move: '{}' {} of set {} move {}",
      self.tile,
      match self.from_where {
        FromWhere::Start => "start",
        FromWhere::End => "end",
      },
      self.set_index,
      match self.move_type {
        MoveType::FromPlayerToBoard => "from player to board",
        MoveType::FromBoardToPlayer => "from board to player",
      },
    )
  }
}
