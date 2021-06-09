use crate::game::{Point, PointStatus, BOARD_SIZE};
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
  Vertical,
  Horizontal,
  Diagonal1, // left top to right bottom
  Diagonal2, // left bottom to right top
}

#[derive(Debug, Clone)]
struct PointGroup {
  d: Direction,
  points: Vec<Point>,
}

impl PointGroup {
  fn new(d: Direction, points: Vec<Point>) -> PointGroup {
    PointGroup { d, points }
  }
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
      status => find_point(status, just_before_point, board),
    };

    if board[next_point.x][next_point.y] == PointStatus::Empty {
      break;
    } else {
      continue;
    }
  }
  return next_point;
}

fn find_point(
  status: PointStatus,
  point: &Point,
  board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) -> Point {
  let mut next_point: Option<Point> = None;

  let point_groups: Vec<PointGroup> = [
    get_continuous_directed_stones(Direction::Vertical, status, point, board),
    get_continuous_directed_stones(Direction::Horizontal, status, point, board),
    get_continuous_directed_stones(Direction::Diagonal1, status, point, board),
    get_continuous_directed_stones(Direction::Diagonal2, status, point, board),
  ]
  .concat();

  println!("{:?} {:?}", status, point_groups);

  // 사방이 빈 경우
  if point_groups.len() == 0 {
    let mut next_x: usize = point.x;
    let mut next_y: usize = point.y;

    if point.x < 18 && point.x > 0 {
      let random = rand::thread_rng().gen_range(0..2);
      if random == 0 {
        next_x += 1;
      } else if random == 1 {
        next_x -= 1;
      }
    }
    if point.y < 18 && point.y > 0 {
      let random = rand::thread_rng().gen_range(0..2);
      if random == 0 {
        next_y += 1;
      } else if random == 1 {
        next_y -= 1;
      }
    }
    println!("next point - 1:{:?}", next_point);
    return Point::new(next_x, next_y);
  }

  // 3개 찾은 경우
  for pg in point_groups.iter() {
    // pg.d 에 따라서 계산식을 고른다.
    // first, last 를 찾아서 그거보다 더 바깥쪽으로 가는 점을 찾는다.
    if pg.points.len() == 3 {
      next_point = get_available_point(pg, status, board);
    }
  }
  if next_point != None {
    println!("next point - 3:{:?}", next_point);
    return next_point.unwrap();
  }

  // 2개 찾은 경우
  for pg in point_groups.iter() {
    // pg.d 에 따라서 계산식을 고른다.
    // first, last 를 찾아서 그거보다 더 바깥쪽으로 가는 점을 찾는다.
    if pg.points.len() == 2 {
      next_point = get_available_point(pg, status, board);
    }
  }

  if next_point != None {
    println!("next point - 2:{:?}", next_point);
    return next_point.unwrap();
  }

  // 계산을 못 한 경우 아무데나 둔다.
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

fn get_available_point(
  pg: &PointGroup,
  status: PointStatus,
  board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) -> Option<Point> {
  let first = pg.points.first().unwrap();
  let last = pg.points.last().unwrap();
  let available_point: Option<Point>;
  let prev: Option<Point>;
  let next: Option<Point>;

  match pg.d {
    Direction::Vertical => {
      prev = if first.y > 0 {
        if board[first.x][first.y - 1] == PointStatus::Empty {
          Some(Point::new(first.x, first.y - 1))
        } else if board[first.x][first.y - 1] == status {
          if first.y > 1 {
            if board[first.x][first.y - 2] == PointStatus::Empty {
              Some(Point::new(first.x, first.y - 2))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
      next = if last.y < 18 {
        if board[last.x][last.y + 1] == PointStatus::Empty {
          Some(Point::new(last.x, last.y + 1))
        } else if board[last.x][last.y + 1] == status {
          if first.y < 17 {
            if board[first.x][first.y + 2] == PointStatus::Empty {
              Some(Point::new(last.x, last.y + 2))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
    }
    Direction::Horizontal => {
      prev = if first.x > 0 {
        if board[first.x - 1][first.y] == PointStatus::Empty {
          Some(Point::new(first.x - 1, first.y))
        } else if board[first.x - 1][first.y] == status {
          if first.x > 1 {
            if board[first.x - 2][first.y] == PointStatus::Empty {
              Some(Point::new(first.x - 2, first.y))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
      next = if last.x < 18 {
        if board[last.x + 1][last.y] == PointStatus::Empty {
          Some(Point::new(last.x + 1, last.y))
        } else if board[last.x + 1][last.y] == status {
          if last.y < 17 {
            if board[first.x + 2][first.y] == PointStatus::Empty {
              Some(Point::new(last.x + 2, last.y))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
    }
    Direction::Diagonal1 => {
      prev = if first.x > 0 && first.y > 0 {
        if board[first.x - 1][first.y - 1] == PointStatus::Empty {
          Some(Point::new(first.x - 1, first.y - 1))
        } else if board[first.x - 1][first.y - 1] == status {
          if first.x > 1 && first.y > 1 {
            if board[first.x - 2][first.y - 2] == PointStatus::Empty {
              Some(Point::new(first.x - 2, first.y - 2))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
      next = if last.x < 18 && last.y < 18 {
        if board[first.x + 1][first.y + 1] == PointStatus::Empty {
          Some(Point::new(first.x + 1, first.y + 1))
        } else if board[first.x + 1][first.y + 1] == status {
          if last.x < 17 && last.y < 17 {
            if board[first.x + 2][first.y + 2] == PointStatus::Empty {
              Some(Point::new(first.x + 2, first.y + 2))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
    }
    Direction::Diagonal2 => {
      prev = if first.x > 0 && first.y < 18 {
        if board[first.x - 1][first.y + 1] == PointStatus::Empty {
          Some(Point::new(first.x - 1, first.y + 1))
        } else if board[first.x - 1][first.y + 1] == status {
          if first.x > 1 && first.y < 17 {
            if board[first.x - 2][first.y + 2] == PointStatus::Empty {
              Some(Point::new(first.x - 2, first.y + 2))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
      next = if last.x < 18 && last.y > 0 {
        if board[first.x + 1][first.y - 1] == PointStatus::Empty {
          Some(Point::new(first.x + 1, first.y - 1))
        } else if board[first.x + 1][first.y - 1] == status {
          if last.x < 17 && last.y > 1 {
            if board[first.x + 2][first.y - 2] == PointStatus::Empty {
              Some(Point::new(first.x + 2, first.y - 2))
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      };
    }
  }

  if prev != None {
    available_point = prev;
  } else {
    available_point = next;
  };

  available_point
}

// 해당 좌표에 어떤 돌이 있는지 확인하여 연속된 돌을 찾는다.
fn get_continuous_directed_stones(
  direction: Direction,
  status: PointStatus,
  p: &Point,
  board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) -> Vec<PointGroup> {
  let points_vectors = get_directed_position(&direction, p);

  let mut point_groups: Vec<PointGroup> = vec![];
  for pv in points_vectors.iter() {
    let mut continuous_points: Vec<Point> = vec![];
    for p in pv.iter() {
      if board[p.x][p.y] == status {
        continuous_points.push(Point::new(p.x, p.y));
      } else {
        break;
      }
    }
    if continuous_points.len() > 1 {
      point_groups.push(PointGroup::new(direction, continuous_points));
    }
  }
  point_groups
}

// 확인해야 할 좌표를 찾는다.
fn get_directed_position(direction: &Direction, p: &Point) -> [Vec<Point>; 2] {
  let mut points_vectors: [Vec<Point>; 2] = [vec![], vec![]];

  match direction {
    Direction::Vertical => {
      if p.y > 1 {
        points_vectors[0].push(Point::new(p.x, p.y - 2));
      }
      if p.y > 0 {
        points_vectors[0].push(Point::new(p.x, p.y - 1));
      }
      points_vectors[0].push(Point::new(p.x, p.y));
      points_vectors[1].push(Point::new(p.x, p.y));
      if p.y < 18 {
        points_vectors[1].push(Point::new(p.x, p.y + 1));
      }
      if p.y < 17 {
        points_vectors[1].push(Point::new(p.x, p.y + 2));
      }
    }
    Direction::Horizontal => {
      if p.x > 1 {
        points_vectors[0].push(Point::new(p.x - 2, p.y));
      }
      if p.x > 0 {
        points_vectors[0].push(Point::new(p.x - 1, p.y));
      }
      points_vectors[0].push(Point::new(p.x, p.y));
      points_vectors[1].push(Point::new(p.x, p.y));
      if p.x < 18 {
        points_vectors[1].push(Point::new(p.x + 1, p.y));
      }
      if p.x < 7 {
        points_vectors[1].push(Point::new(p.x + 2, p.y));
      }
    }
    Direction::Diagonal1 => {
      // diagonal (left top to right bottom)
      if p.x > 1 && p.y > 1 {
        points_vectors[0].push(Point::new(p.x - 2, p.y - 2));
      }
      if p.x > 0 && p.y > 0 {
        points_vectors[0].push(Point::new(p.x - 1, p.y - 1));
      }
      points_vectors[0].push(Point::new(p.x, p.y));
      points_vectors[1].push(Point::new(p.x, p.y));
      if p.x < 18 && p.y < 18 {
        points_vectors[1].push(Point::new(p.x + 1, p.y + 1));
      }
      if p.x < 17 && p.y < 17 {
        points_vectors[1].push(Point::new(p.x + 2, p.y + 2));
      }
    }
    Direction::Diagonal2 => {
      // diagonal (left bottom to right top)
      if p.x > 1 && p.y < 17 {
        points_vectors[0].push(Point::new(p.x - 2, p.y + 2));
      }
      if p.x > 0 && p.y < 18 {
        points_vectors[0].push(Point::new(p.x - 1, p.y + 1));
      }
      points_vectors[0].push(Point::new(p.x, p.y));
      points_vectors[1].push(Point::new(p.x, p.y));
      if p.x < 18 && p.y > 0 {
        points_vectors[1].push(Point::new(p.x + 1, p.y - 1));
      }
      if p.x < 17 && p.y > 1 {
        points_vectors[1].push(Point::new(p.x + 2, p.y - 2));
      }
    }
  };
  points_vectors
}

pub fn check_game_end(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> bool {
  false
}
