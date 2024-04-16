use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Black,
}

#[derive(Debug,Deserialize)]
pub enum TileValue {
    Number(u8),
    Joker,
}

#[derive(Debug,Deserialize)]
pub struct Tile {
    value: TileValue,
    color: Color,
}