use crate::game::{Board, Color, Direction, Point, PointStatus};
// mod find_point_v1;
// mod random;
mod find_point_v2;

pub fn find_next_point(just_before_point: &Point, color: &Color, board: &Board) -> Point {
  match board.status_at(just_before_point) {
    PointStatus::Empty => {
      // 내가 처음 두는 경우. 보드가 비어있으므로 가운데에 둔다.
      Point::new(9, 9)
    }
    _ => {
      let (h_left, h_right) = board.get_directed_points(just_before_point, Direction::Horizontal);
      let (v_left, v_right) = board.get_directed_points(just_before_point, Direction::Vertical);
      let (i_left, i_right) = board.get_directed_points(just_before_point, Direction::Increase);
      let (d_left, d_right) = board.get_directed_points(just_before_point, Direction::Decrease);

      println!("{:?} {:?}", h_left, h_right);
      println!("{:?} {:?}", v_left, v_right);
      println!("{:?} {:?}", i_left, i_right);
      println!("{:?} {:?}", d_left, d_right);
      find_point_v2::find(color, just_before_point, board)
    }
  }
}
