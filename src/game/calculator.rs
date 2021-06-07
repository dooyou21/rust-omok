use crate::game::{Color, Point, PointStatus, BOARD_SIZE};
use rand::prelude::*;

enum Direction {
  Vertical,
  Horizontal,
  DiagonalLeftBottom,
  DiagonalRightBottom,
}

pub fn find_next_point(color: &Color, board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> Point {
  let mut x = rand::thread_rng().gen_range(0..19);
  let mut y = rand::thread_rng().gen_range(0..19);

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

  // 5. 보드가 비어있는 경우9,9(가운데)에 둔다.

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
