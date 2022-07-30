use serde::Serialize;

#[derive(Clone, Serialize, Debug, PartialEq)]
pub enum Color {
  White,
  Black,
}

#[derive(Clone, Serialize, Debug, PartialEq)]
pub enum Rank {
  Pawn,
  Bishop,
  Queen,
  King,
  Knight,
  Rook,
}


#[derive(Clone, Serialize, Debug)]
pub struct Piece {
  pub color: Color,
  pub rank: Rank,
}

impl Piece {
  pub fn white(rank: Rank) -> Self {
    Self {color: Color::White, rank}
  }
  
  pub fn black(rank: Rank) -> Self {
    Self {color: Color::Black, rank}
  }
}
