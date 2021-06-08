use crate::game::{Color, Point, PointStatus, BOARD_SIZE};
// use rand::prelude::*;

enum Direction {
  Vertical,
  Horizontal,
  Diagonal1, // left top to right bottom
  Diagonal2, // left bottom to right top
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
      PointStatus::Black => {
        let vertical: Vec<Point> = get_continuous_stone(
          Direction::Vertical,
          PointStatus::Black,
          just_before_point,
          board,
        );
        let horizontal: Vec<Point> = get_continuous_stone(
          Direction::Horizontal,
          PointStatus::Black,
          just_before_point,
          board,
        );
        let diagonal1: Vec<Point> = get_continuous_stone(
          Direction::Diagonal1,
          PointStatus::Black,
          just_before_point,
          board,
        );
        let diagonal2: Vec<Point> = get_continuous_stone(
          Direction::Diagonal2,
          PointStatus::Black,
          just_before_point,
          board,
        );

        Point::new(just_before_point.x + 1, just_before_point.y + 1)
      }
      PointStatus::White => {
        // white 돌멩이에 대하여 1234 검사 및 확인.

        Point::new(just_before_point.x + 1, just_before_point.y + 1)
      }
    };

    // 1. 동일 방향으로 인접한 돌멩이가 4개인 것이 있는가?
    // 1-1. 나와 동일한 색인가? 4개와 인접한 곳에 둔다.

    // 2. 동일 방향으로 인접한 돌멩이가 3개인 것이 있는가? 있으면 양 끝의 점이 후보가 된다.
    // 좌/상 or 우/하 중에 어떻게 선택할 것인가?
    // 1) 같은 방향으로 인접한 돌멩이가 있는 곳을 고른다
    // 2) 주변 8방위 안에 인접한 돌멩이가 있는 쪽에 둔다 (더 많은 쪽에 둔다)
    // 모든 조건이 동일한 경우 10,10에 가까운 점에 둔다.

    // 3. 동일 방향으로 인접한 돌멩이가 2개인 것이 있는가? 있으면 양 끝점이 후보가 된다.
    // 주변 8방위에 상대방 색의 돌멩이 하나당 1점으로 계산하여 점수가 낮은 것을 선택한다.
    // 모든 조건이 동일한 경우 10,10에 가까운 점에 둔다.

    // 4. 나와 동일한 색의 점을 찾고, 8방위 중에서 랜덤으로 한 자리를 골라서 둔다.

    if board[next_point.x][next_point.y] == PointStatus::Empty {
      // x = rand::thread_rng().gen_range(0..19);
      // y = rand::thread_rng().gen_range(0..19);
      break;
    } else {
      continue;
    }
  }
  return next_point;
}

fn get_directed_position(direction: Direction, p: &Point) -> Vec<Point> {
  match direction {
    Direction::Vertical => {
      let mut vec = vec![];
      vec.push(Point::new(p.x, p.y - 1));
      vec.push(Point::new(p.x, p.y + 1));
      vec
    }
    Direction::Horizontal => {
      let mut vec = vec![];
      vec.push(Point::new(p.x - 1, p.y));
      vec.push(Point::new(p.x + 1, p.y));
      vec
    }
    Direction::Diagonal1 => {
      // diagonal (left top to right bottom)
      let mut vec = vec![];
      vec.push(Point::new(p.x - 1, p.y - 1));
      vec.push(Point::new(p.x + 1, p.y + 1));
      vec
    }
    Direction::Diagonal2 => {
      // diagonal (left bottom to right top)
      let mut vec = vec![];
      vec.push(Point::new(p.x - 1, p.y + 1));
      vec.push(Point::new(p.x + 1, p.y - 1));
      vec
    }
  }
}

fn get_continuous_stone(
  direction: Direction,
  status: PointStatus,
  p: &Point,
  board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) -> Vec<Point> {
  let mut vec: Vec<Point> = vec![];
  let available_position = get_directed_position(direction, p);
  if board[available_position[0].x][available_position[0].y] == status {
    vec.push(available_position[0]);
  }
  vec.push(Point::new(p.x, p.y));
  if board[available_position[1].x][available_position[1].y] == status {
    vec.push(available_position[1]);
  }
  vec
}

// fn is_empty(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> bool {
//   board.into_iter().flatten().all(|&ps| match ps {
//     PointStatus::Empty => true,
//     PointStatus::Black => false,
//     PointStatus::White => false,
//   })
// }

pub fn check_game_end(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> bool {
  // TODO: 게임이 끝났는지 판단할 수 있어야 함.
  false
}
