use crate::game::{Board, Color, Point, PointStatus};
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

pub fn find_next_point(just_before_point: &Point, color: &Color, board: &Board) -> Point {
  match board.check(just_before_point) {
    PointStatus::Empty => {
      // 내가 처음 두는 경우. 보드가 비어있으므로 가운데에 둔다.
      Point::new(9, 9)
    }
    _ => find_point_v2::find(color, just_before_point, board),
  }
}
