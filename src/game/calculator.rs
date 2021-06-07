use crate::game::{Color, Point, PointStatus, BOARD_SIZE};
use rand::prelude::*;

pub fn find_next_point(color: &Color, board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> Point {
  let mut x = rand::thread_rng().gen_range(0..19);
  let mut y = rand::thread_rng().gen_range(0..19);

  while board[x][y] != PointStatus::Empty {
    x = rand::thread_rng().gen_range(0..19);
    y = rand::thread_rng().gen_range(0..19);
  }

  Point::new(x, y)
}

pub fn check_game_end(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> bool {
  // TODO: 게임이 끝났는지 판단할 수 있어야 함.
  false
}
