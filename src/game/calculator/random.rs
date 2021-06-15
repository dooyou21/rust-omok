use crate::game::{Point, PointStatus, BOARD_SIZE};
use rand::prelude::*;

pub fn find(
  status: PointStatus,
  point: &Point,
  board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) -> Point {
  let mut x: usize;
  let mut y: usize;
  loop {
    x = rand::thread_rng().gen_range(0..19);
    y = rand::thread_rng().gen_range(0..19);
    if board[x][y] != PointStatus::Empty {
      continue;
    } else {
      break;
    }
  }

  println!("next point - random:{:?}", Point::new(x, y));
  Point::new(x, y)
}
