#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#![allow(unused_variables)]

use tauri::State;

use std::{sync::RwLock};

mod piece;
mod board;

use board::{Board, Matrix, Pos};
use piece::{Piece, Color};

fn main() {
  tauri::Builder::default()
    .manage(RwLock::new(Board::new()))
    .invoke_handler(tauri::generate_handler![get_state, move_piece, reset, get_steps])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_state(state: State<RwLock<Board>>) -> Matrix<Piece> {
  return state.read().unwrap().pieces.clone()
}

#[tauri::command]
fn reset(state: State<RwLock<Board>>) {
  return *state.write().unwrap() = Board::new();
}

#[tauri::command]
fn move_piece(state: State<RwLock<Board>>, from: Pos, to: Pos) {
  let mut board = state.write().unwrap();
  board.move_piece(from, to);
}

#[tauri::command]
fn get_steps(state: State<RwLock<Board>>, pos: Pos) -> Option<Vec<Pos>> {
  let board = state.read().unwrap();
  let piece = board.pieces[pos.y][pos.x].as_ref()?;

  let mut steps = vec![];
  let step_container = if piece.color == Color::White {&board.white_attacks} else {&board.black_attacks};

  for y in 0..step_container.len() {
    for x in 0..step_container[y].len() {
      if let Some(pieces) = &step_container[y][x] {
        if pieces.contains(&pos) {
          steps.push(Pos::new(x, y));
        }
      }
    }
  }

  return Some(steps);
}