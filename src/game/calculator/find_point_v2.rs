use crate::game::{Board, Color, Direction, Point, PointStatus, BOARD_SIZE};
use rand::prelude::*;
use std::cmp::Reverse;

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
  fn get(&self, point: &Point) -> u16 {
    self.board[point.y][point.x]
  }
}

enum ScoreMode {
  Attack,
  Defence,
}

struct ScoreCalculator {
  attack_score_map: Vec<(Vec<PointStatus>, Vec<PointStatus>, u16)>, // (left, right, point);
  defence_score_map: Vec<(Vec<PointStatus>, Vec<PointStatus>, u16)>, // (left, right, point);
}

impl ScoreCalculator {
  fn new(color: &Color) -> ScoreCalculator {
    let (i, u) = match color {
      Color::Black => (PointStatus::Black, PointStatus::White),
      Color::White => (PointStatus::White, PointStatus::Black),
    };

    let mut attack_score_map = make_score_board(i, u, ScoreMode::Attack);
    let mut defence_score_map = make_score_board(u, i, ScoreMode::Defence);
    attack_score_map.sort_by_key(|a| Reverse(a.2));
    defence_score_map.sort_by_key(|a| Reverse(a.2));

    ScoreCalculator {
      attack_score_map,
      defence_score_map,
    }
  }
  fn get_score(&self, left: &Vec<PointStatus>, right: &Vec<PointStatus>, mode: ScoreMode) -> u16 {
    let score_board = match mode {
      ScoreMode::Attack => &self.attack_score_map,
      ScoreMode::Defence => &self.defence_score_map,
    };

    let mut score = 0;
    for (l, r, s) in score_board.iter() {
      let mut l_found;
      if l.is_empty() {
        l_found = true;
      } else {
        if left.len() >= l.len() {
          l_found = true;
          for (i, ps) in l.iter().enumerate() {
            let check_target_ps = left.get(i).unwrap();
            if ps != check_target_ps {
              l_found = false;
              break;
            }
          }
        } else {
          continue;
        }
      }

      let mut r_found;
      if r.is_empty() {
        r_found = true;
      } else {
        if right.len() >= r.len() {
          r_found = true;
          for (i, ps) in r.iter().enumerate() {
            let check_target_ps = right.get(i).unwrap();
            if ps != check_target_ps {
              r_found = false;
              break;
            }
          }
        } else {
          continue;
        }
      }

      if l_found && r_found {
        score = *s;
        break;
      }
    }

    score
  }
}

pub fn find(just_before_point: &Point, color: &Color, board: &Board) -> Point {
  let mut score_board = ScoreBoard::new();
  let score_calculator = ScoreCalculator::new(color);

  for x in 0..BOARD_SIZE {
    for y in 0..BOARD_SIZE {
      let anchor = Point::new(x, y);
      if board.status_at(&anchor) != PointStatus::Empty {
        score_board.set(&anchor, 0);
        continue;
      }

      let mut score = 0;
      let (h_left, h_right) = board.get_directed_points(&anchor, Direction::Horizontal);
      score += score_calculator.get_score(&h_left, &h_right, ScoreMode::Attack);
      score += score_calculator.get_score(&h_left, &h_right, ScoreMode::Defence);
      let (v_left, v_right) = board.get_directed_points(&anchor, Direction::Vertical);
      score += score_calculator.get_score(&v_left, &v_right, ScoreMode::Attack);
      score += score_calculator.get_score(&v_left, &v_right, ScoreMode::Defence);
      let (i_left, i_right) = board.get_directed_points(&anchor, Direction::Increase);
      score += score_calculator.get_score(&i_left, &i_right, ScoreMode::Attack);
      score += score_calculator.get_score(&i_left, &i_right, ScoreMode::Defence);
      let (d_left, d_right) = board.get_directed_points(&anchor, Direction::Decrease);
      score += score_calculator.get_score(&d_left, &d_right, ScoreMode::Attack);
      score += score_calculator.get_score(&d_left, &d_right, ScoreMode::Defence);

      score_board.set(&anchor, score);
    }
  }

  for line in &score_board.board {
    println!("{:?}", line);
  }

  let mut highest_score: (u16, Point) = (0, Point::new(0, 0));
  let mut highest_score_points = vec![];

  for x in 0..BOARD_SIZE {
    for y in 0..BOARD_SIZE {
      let point = Point::new(x, y);
      let score = score_board.get(&point);
      if score > highest_score.0 {
        highest_score = (score, point);
        highest_score_points = vec![(score, point)];
      } else if score == highest_score.0 {
        highest_score_points.push((score, point));
      }
    }
  }

  if highest_score_points.len() > 1 {
    let index = rand::thread_rng().gen_range(0..highest_score_points.len());
    highest_score_points[index].1
  } else if highest_score_points.len() == 1 {
    highest_score_points[0].1
  } else {
    let points = board.get_available_near_points(just_before_point);
    let index = rand::thread_rng().gen_range(0..points.len());
    points[index]
  }
}

fn make_score_board(
  i: PointStatus,
  u: PointStatus,
  mode: ScoreMode,
) -> Vec<(Vec<PointStatus>, Vec<PointStatus>, u16)> {
  let e = PointStatus::Empty;
  match mode {
    ScoreMode::Attack => {
      vec![
        // =====4개
        (
          vec![],
          vec![i.clone(), i.clone(), i.clone(), i.clone()],
          1000,
        ),
        (vec![i.clone()], vec![i.clone(), i.clone(), i.clone()], 1000),
        (vec![i.clone(), i.clone()], vec![i.clone(), i.clone()], 1000),
        (vec![i.clone(), i.clone(), i.clone()], vec![i.clone()], 1000),
        (
          vec![i.clone(), i.clone(), i.clone(), i.clone()],
          vec![],
          1000,
        ),
        // =====3개
        (
          vec![e.clone()],
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          75,
        ),
        (
          vec![e.clone()],
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          55,
        ),
        (
          vec![u.clone()],
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          55,
        ),
        (
          vec![u.clone()],
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          30,
        ),
        //
        (
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          75,
        ),
        (
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          vec![u.clone()],
          55,
        ),
        (
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          vec![e.clone()],
          55,
        ),
        (
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          vec![u.clone()],
          30,
        ),
        //
        (
          vec![i.clone(), u.clone()],
          vec![i.clone(), i.clone(), u.clone()],
          30,
        ),
        (
          vec![i.clone(), u.clone()],
          vec![i.clone(), i.clone(), e.clone()],
          55,
        ),
        (
          vec![i.clone(), e.clone()],
          vec![i.clone(), i.clone(), u.clone()],
          55,
        ),
        (
          vec![i.clone(), e.clone()],
          vec![i.clone(), i.clone(), e.clone()],
          75,
        ),
        //
        (
          vec![i.clone(), i.clone(), u.clone()],
          vec![i.clone(), u.clone()],
          30,
        ),
        (
          vec![i.clone(), i.clone(), u.clone()],
          vec![i.clone(), e.clone()],
          55,
        ),
        (
          vec![i.clone(), i.clone(), e.clone()],
          vec![i.clone(), u.clone()],
          55,
        ),
        (
          vec![i.clone(), i.clone(), e.clone()],
          vec![i.clone(), e.clone()],
          75,
        ),
        //
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          vec![u.clone()],
          57,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          vec![e.clone()],
          57,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          vec![u.clone()],
          65,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          80,
        ),
        //
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          57,
        ),
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          65,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          57,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          80,
        ),
        // =====2개
        (vec![u.clone()], vec![i.clone(), i.clone(), u.clone()], 5),
        (vec![u.clone()], vec![i.clone(), i.clone(), e.clone()], 35),
        (vec![e.clone()], vec![i.clone(), i.clone(), u.clone()], 35),
        (vec![e.clone()], vec![i.clone(), i.clone(), e.clone()], 45),
        //
        (vec![i.clone(), u.clone()], vec![i.clone(), u.clone()], 5),
        (vec![i.clone(), u.clone()], vec![i.clone(), e.clone()], 35),
        (vec![i.clone(), e.clone()], vec![i.clone(), u.clone()], 35),
        (vec![i.clone(), e.clone()], vec![i.clone(), e.clone()], 45),
        //
        (vec![i.clone(), i.clone(), u.clone()], vec![u.clone()], 5),
        (vec![i.clone(), i.clone(), u.clone()], vec![e.clone()], 35),
        (vec![i.clone(), i.clone(), e.clone()], vec![u.clone()], 35),
        (vec![i.clone(), i.clone(), e.clone()], vec![e.clone()], 45),
        //
        (
          vec![e.clone(), i.clone(), i.clone(), u.clone()],
          vec![u.clone()],
          10,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          vec![u.clone()],
          20,
        ),
        (
          vec![u.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          38,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          50,
        ),
        //
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), u.clone()],
          10,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), u.clone()],
          38,
        ),
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          20,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          50,
        ),
        // =====1개
        (vec![u.clone()], vec![i.clone(), u.clone()], 0),
        (vec![u.clone()], vec![i.clone(), e.clone()], 5),
        (vec![e.clone()], vec![i.clone(), u.clone()], 10),
        (vec![e.clone()], vec![i.clone(), e.clone()], 30),
        //
        (vec![i.clone(), u.clone()], vec![u.clone()], 0),
        (vec![i.clone(), u.clone()], vec![e.clone()], 10),
        (vec![i.clone(), e.clone()], vec![u.clone()], 5),
        (vec![i.clone(), e.clone()], vec![e.clone()], 30),
      ]
    }
    ScoreMode::Defence => {
      vec![
        // =====4개
        (
          vec![],
          vec![i.clone(), i.clone(), i.clone(), i.clone()],
          500,
        ),
        (vec![i.clone()], vec![i.clone(), i.clone(), i.clone()], 2000),
        (vec![i.clone(), i.clone()], vec![i.clone(), i.clone()], 2000),
        (vec![i.clone(), i.clone(), i.clone()], vec![i.clone()], 2000),
        (
          vec![i.clone(), i.clone(), i.clone(), i.clone()],
          vec![],
          500,
        ),
        // =====3개
        (
          vec![e.clone()],
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          100,
        ),
        (
          vec![e.clone()],
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          65,
        ),
        (
          vec![u.clone()],
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          50,
        ),
        (
          vec![u.clone()],
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          0,
        ),
        //
        (
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          100,
        ),
        (
          vec![i.clone(), i.clone(), i.clone(), e.clone()],
          vec![u.clone()],
          50,
        ),
        (
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          vec![e.clone()],
          65,
        ),
        (
          vec![i.clone(), i.clone(), i.clone(), u.clone()],
          vec![u.clone()],
          0,
        ),
        //
        (
          vec![i.clone(), u.clone()],
          vec![i.clone(), i.clone(), u.clone()],
          0,
        ),
        (
          vec![i.clone(), u.clone()],
          vec![i.clone(), i.clone(), e.clone()],
          30,
        ),
        (
          vec![i.clone(), e.clone()],
          vec![i.clone(), i.clone(), u.clone()],
          32,
        ),
        (
          vec![i.clone(), e.clone()],
          vec![i.clone(), i.clone(), e.clone()],
          70,
        ),
        //
        (
          vec![i.clone(), i.clone(), u.clone()],
          vec![i.clone(), u.clone()],
          0,
        ),
        (
          vec![i.clone(), i.clone(), u.clone()],
          vec![i.clone(), e.clone()],
          32,
        ),
        (
          vec![i.clone(), i.clone(), e.clone()],
          vec![i.clone(), u.clone()],
          30,
        ),
        (
          vec![i.clone(), i.clone(), e.clone()],
          vec![i.clone(), e.clone()],
          70,
        ),
        //
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          vec![u.clone()],
          68,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          vec![e.clone()],
          68,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          vec![u.clone()],
          40,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          30,
        ),
        //
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          68,
        ),
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          40,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), u.clone()],
          68,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), i.clone(), e.clone()],
          30,
        ),
        // =====2개
        (vec![u.clone()], vec![i.clone(), i.clone(), u.clone()], 0),
        (vec![u.clone()], vec![i.clone(), i.clone(), e.clone()], 5),
        (vec![e.clone()], vec![i.clone(), i.clone(), u.clone()], 10),
        (vec![e.clone()], vec![i.clone(), i.clone(), e.clone()], 30),
        //
        (vec![i.clone(), u.clone()], vec![i.clone(), u.clone()], 0),
        (vec![i.clone(), u.clone()], vec![i.clone(), e.clone()], 8),
        (vec![i.clone(), e.clone()], vec![i.clone(), u.clone()], 8),
        (vec![i.clone(), e.clone()], vec![i.clone(), e.clone()], 30),
        //
        (vec![i.clone(), i.clone(), u.clone()], vec![u.clone()], 0),
        (vec![i.clone(), i.clone(), u.clone()], vec![e.clone()], 10),
        (vec![i.clone(), i.clone(), e.clone()], vec![u.clone()], 5),
        (vec![i.clone(), i.clone(), e.clone()], vec![e.clone()], 30),
        //
        (
          vec![e.clone(), i.clone(), i.clone(), u.clone()],
          vec![u.clone()],
          0,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          vec![u.clone()],
          3,
        ),
        (
          vec![u.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          25,
        ),
        (
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          vec![e.clone()],
          4,
        ),
        //
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), u.clone()],
          0,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), u.clone()],
          25,
        ),
        (
          vec![u.clone()],
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          3,
        ),
        (
          vec![e.clone()],
          vec![e.clone(), i.clone(), i.clone(), e.clone()],
          4,
        ),
        // =====1개
        (vec![u.clone()], vec![i.clone(), u.clone()], 0),
        (vec![u.clone()], vec![i.clone(), e.clone()], 2),
        (vec![e.clone()], vec![i.clone(), u.clone()], 5),
        (vec![e.clone()], vec![i.clone(), e.clone()], 30),
        //
        (vec![i.clone(), u.clone()], vec![u.clone()], 0),
        (vec![i.clone(), u.clone()], vec![e.clone()], 5),
        (vec![i.clone(), e.clone()], vec![u.clone()], 2),
        (vec![i.clone(), e.clone()], vec![e.clone()], 30),
      ]
    }
  }
}
