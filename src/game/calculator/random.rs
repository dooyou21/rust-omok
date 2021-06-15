use crate::game::{Point, PointStatus, BOARD_SIZE};
use rand::prelude::*;

pub fn find(color: &Color, point: &Point, board: &Board) -> Point {
  let mut next_point: Point;
  loop {
    let x = rand::thread_rng().gen_range(0..19);
    let y = rand::thread_rng().gen_range(0..19);
    next_point = Point::new(x, y);
    if board.check(&next_point) != PointStatus::Empty {
      continue;
    } else {
      break;
    }
  }

  next_point
}
