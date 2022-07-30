use serde::{Serialize, Deserialize};
use std::mem;
use crate::piece::{Piece, Rank::*, Color::{self, *}};

pub type Matrix<T> = [[Option<T>; 8]; 8];

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Pos {
  pub x: usize,
  pub y: usize
}

impl Pos {
  pub fn new(x: usize, y: usize) -> Self {
    Self {x, y}
  }
}

impl From<(i32, i32)> for Pos {
  fn from(from: (i32, i32)) -> Self {
    Pos {
      x: from.0 as usize,
      y: from.1 as usize,
    }
  }
}

#[derive(Clone, Serialize)]
pub struct Board {
  pub pieces: Matrix<Piece>,
  pub white_attacks: Matrix<Vec<Pos>>,
  pub black_attacks: Matrix<Vec<Pos>>,
}

impl Board {
  pub fn new() -> Self {
    let mut pieces: Matrix<Piece> = Default::default();
    pieces[0] = [Piece::black(Rook), Piece::black(Knight), Piece::black(Bishop), Piece::black(Queen), Piece::black(King), Piece::black(Bishop), Piece::black(Knight), Piece::black(Rook)].map(|p| Some(p));
    pieces[1] = [Piece::black(Pawn), Piece::black(Pawn), Piece::black(Pawn), Piece::black(Pawn), Piece::black(Pawn), Piece::black(Pawn), Piece::black(Pawn), Piece::black(Pawn)].map(|p| Some(p));
    pieces[6] = [Piece::white(Pawn), Piece::white(Pawn), Piece::white(Pawn), Piece::white(Pawn), Piece::white(Pawn), Piece::white(Pawn), Piece::white(Pawn), Piece::white(Pawn)].map(|p| Some(p));
    pieces[7] = [Piece::white(Rook), Piece::white(Knight), Piece::white(Bishop), Piece::white(Queen), Piece::white(King), Piece::white(Bishop), Piece::white(Knight), Piece::white(Rook)].map(|p| Some(p));

    let mut board = Self {
      pieces,
      white_attacks: Default::default(),
      black_attacks: Default::default(),
    };

    board.calc_attacks();

    return board;
  }

  pub fn move_piece(&mut self, from: Pos, to: Pos) {
    let attacker = mem::take(&mut self.pieces[from.y][from.x]);
    self.pieces[to.y][to.x] = attacker;

    self.calc_attacks();
  }

  fn calc_attacks(&mut self) {
    self.white_attacks = Default::default();
    self.black_attacks = Default::default();

    for y in 0..self.pieces.len() {
      for x in 0..self.pieces[y].len() {
        self.calc_attack(x, y);
      } 
    }
  }

  fn calc_attack(&mut self, x: usize, y: usize) {
    if let Some(piece) = &self.pieces[y][x] {
      let valid_steps = match piece.rank {
        Pawn => self.calc_pawn(x, y, &piece.color),
        Knight => self.calc_knight(x, y, &piece.color),
        Bishop => self.calc_bishop(x, y, &piece.color),
        Rook => self.calc_rook(x, y, &piece.color),
        Queen => self.calc_queen(x, y, &piece.color),
        _ => vec![]
      };
    
      let container = match piece.color {
        White => &mut self.white_attacks,
        Black => &mut self.black_attacks,
      };
    
      for Pos{x: step_x, y: step_y} in valid_steps {
        let target = container[step_y][step_x].get_or_insert(vec![]);
        target.push(Pos::new(x, y));
      }
    }
  }

  fn calc_knight(&self, x: usize, y: usize, color: &Color) -> Vec<Pos> {
    let x = x as i32;
    let y = y as i32;

    let all_steps = [(x-2, y+1), (x-2, y-1), (x+2, y+1), (x+2, y-1), (x+1, y-2), (x-1, y-2), (x+1, y+2), (x-1, y+2)];

    let valid_steps = all_steps
      .into_iter()
      .filter(|&(x, y)| (0..8).contains(&x) && (0..8).contains(&y))
      .map(|(x, y)| Pos::new(x as usize, y as usize))
      .filter(|&Pos{x, y}| {
        match &self.pieces[y][x] {
          Some(Piece{color: c, rank}) if c == color => false,
          _ => true,
        }
      })
      .collect();

    return valid_steps;
  }

  fn calc_bishop(&self, x: usize, y: usize, color: &Color) -> Vec<Pos> {
    return self.get_diagonal_slide_steps(x, y, color);
  }

  fn calc_rook(&self, x: usize, y: usize, color: &Color) -> Vec<Pos> {
    return self.get_straight_slide_steps(x, y, color);
  }

  fn calc_queen(&self, x: usize, y: usize, color: &Color) -> Vec<Pos> {
    let mut straight_steps = self.get_straight_slide_steps(x, y, color);
    let mut diagonal_steps = self.get_diagonal_slide_steps(x, y, color);

    straight_steps.append(&mut diagonal_steps);
    return straight_steps;
  }

  fn get_diagonal_slide_steps(&self, x: usize, y: usize, color: &Color) -> Vec<Pos> {
    let pos = Pos::new(x, y);

    let mut valid_steps = vec![];

    for x in [1, -1] {
      for y in [-1, 1] {
        let mut steps = self.get_slide_steps(&pos, x, y, color);
        valid_steps.append(&mut steps);
      }
    }

    return valid_steps
  }

  fn get_straight_slide_steps(&self, x: usize, y: usize, color: &Color) -> Vec<Pos> {
    let pos = Pos::new(x, y);

    return [(1, 0), (-1, 0), (0, 1), (0, -1)]
      .map(|(x, y)| self.get_slide_steps(&pos, x, y, color))
      .concat();
  }

  fn get_slide_steps(&self, pos: &Pos, dir_x: i32, dir_y: i32, color: &Color) -> Vec<Pos> {
    let mut valid_steps = vec![];

    let mut curr_x = pos.x as i32;
    let mut curr_y = pos.y as i32;

    let reason = loop {
      curr_x += dir_x;
      curr_y += dir_y;

      if !(0..8).contains(&curr_x) || !(0..8).contains(&curr_y) {
        break None
      }

      if let Some(piece) = &self.pieces[curr_y as usize][curr_x as usize] {
        break Some(piece)
      }

      valid_steps.push((curr_x, curr_y).into());
    };

    if let Some(piece) = reason {
      if piece.color != *color {
        valid_steps.push((curr_x, curr_y).into());
      }
    }

    return valid_steps;
  }

  fn calc_pawn(&self, x: usize, y: usize, color: &Color) -> Vec<Pos> {
    if y == 7 || 7 == 0 {return vec![]}
    
    let mut valid_steps = vec![];

    let next_y = if *color == White {y-1} else {y+1};
    self.add_if_empty(&mut valid_steps, Pos::new(x, next_y));

    let is_on_base = if *color == White {y == 6} else {y ==1};
    if is_on_base && valid_steps.len() == 1 {
      let next_y = if *color == White {next_y-1} else {next_y+1};
      self.add_if_empty(&mut valid_steps, Pos::new(x, next_y));
    }

    if x > 0 {
      let next_x = x-1;
      self.add_if_enemy(&mut valid_steps, Pos::new(next_x, next_y), color);
    }

    if x < 7 {
      let next_x = x+1;
      self.add_if_enemy(&mut valid_steps, Pos::new(next_x, next_y), color);
    }

    return valid_steps
  }

  fn add_if_empty(&self, container: &mut Vec<Pos>, pos: Pos) {
    if self.pieces[pos.y][pos.x].is_none() {
      container.push(pos)
    }
  }

  fn add_if_enemy(&self, container: &mut Vec<Pos>, pos: Pos, color: &Color) {
    if let Some(piece) = &self.pieces[pos.y][pos.x] {
      if piece.color != *color {
        container.push(pos)
      }
    }
  }
}