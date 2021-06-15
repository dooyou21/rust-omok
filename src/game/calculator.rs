use crate::game::{Point, PointStatus, BOARD_SIZE};
// mod find_point_v1;
// mod random;
mod find_point_v2;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Vertical,
  Horizontal,
  Decrease, // left top to right bottom
  Increase, // left bottom to right top
}

pub fn find_next_point(
  just_before_point: &Point,
  board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) -> Point {
  let mut next_point;
  loop {
    next_point = match board[just_before_point.x][just_before_point.y] {
      PointStatus::Empty => {
        // 내가 처음 두는 경우. 보드가 비어있으므로 가운데에 둔다.
        Point::new(9, 9)
      }
      status => find_point_v2::find(status, just_before_point, board),
    };

    if board[next_point.x][next_point.y] == PointStatus::Empty {
      break;
    } else {
      continue;
    }
  }
  return next_point;
}
