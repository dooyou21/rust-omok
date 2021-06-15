use crate::game::{Board, Color, Direction, Point, PointStatus, BOARD_SIZE};
use rand::prelude::*;

struct ScoreBoard {
  board: [[u16; BOARD_SIZE]; BOARD_SIZE],
}

impl ScoreBoard {
  fn new() -> ScoreBoard {
    ScoreBoard {
      board: [[0; BOARD_SIZE]; BOARD_SIZE],
    }
  }
  fn set(&mut self, point: &Point, score: u16) {
    self.board[point.y][point.x] = score;
  }
}

pub fn find(just_before_point: &Point, color: &Color, board: &Board) -> Point {
  let mut score_board = ScoreBoard::new();

  for x in 0..BOARD_SIZE {
    for y in 0..BOARD_SIZE {
      let anchor = Point::new(x, y);
      score_board.set(&anchor, get_point(&anchor, color, board));
    }
  }

  let mut next_point: Point;
  loop {
    let x = rand::thread_rng().gen_range(0..19);
    let y = rand::thread_rng().gen_range(0..19);
    next_point = Point::new(x, y);
    if board.status_at(&next_point) != PointStatus::Empty {
      continue;
    } else {
      break;
    }
  }
  next_point
}

fn get_point(anchor: &Point, color: &Color, board: &Board) -> u16 {
  let mut point = 0;
  let (h_left, h_right) = board.get_directed_points(anchor, Direction::Horizontal);
  let (v_left, v_right) = board.get_directed_points(anchor, Direction::Vertical);
  let (i_left, i_right) = board.get_directed_points(anchor, Direction::Increase);
  let (d_left, d_right) = board.get_directed_points(anchor, Direction::Decrease);

  // println!("{:?} {:?}", h_left, h_right);
  // println!("{:?} {:?}", v_left, v_right);
  // println!("{:?} {:?}", i_left, i_right);
  // println!("{:?} {:?}", d_left, d_right);

  // TODO: pointMap 에서 left/right를 대조해서 점수를 계산한다.

  point
}
