use crate::game::{PointStatus, BOARD_SIZE};

pub fn check_game_end(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> Option<PointStatus> {
  // Horizontal
  for i in 0..BOARD_SIZE {
    match is_continuous_stone_exist(board[i].to_vec()) {
      Some(point_status) => {
        return Some(point_status);
      }
      None => {}
    }
  }

  // Vertical
  for j in 0..BOARD_SIZE {
    let mut stones: Vec<PointStatus> = vec![];
    for i in 0..BOARD_SIZE {
      stones.push(board[i][j].clone());
    }
    match is_continuous_stone_exist(stones) {
      Some(point_status) => {
        return Some(point_status);
      }
      None => {}
    }
  }

  // Decrease
  for j in 0..BOARD_SIZE {
    let mut _j = j;
    let mut stones: Vec<PointStatus> = vec![];
    for i in 0..BOARD_SIZE {
      stones.push(board[i][_j].clone());
      if _j == BOARD_SIZE - 1 {
        break;
      }
      _j += 1;
    }

    if stones.len() < 5 {
      continue;
    }

    match is_continuous_stone_exist(stones) {
      Some(point_status) => {
        return Some(point_status);
      }
      None => {}
    }
  }
  for i in 0..BOARD_SIZE {
    let mut _i = i;
    let mut stones: Vec<PointStatus> = vec![];
    for j in 0..BOARD_SIZE {
      stones.push(board[_i][j].clone());
      if _i == BOARD_SIZE - 1 {
        break;
      }
      _i += 1;
    }

    if stones.len() < 5 {
      continue;
    }

    match is_continuous_stone_exist(stones) {
      Some(point_status) => {
        return Some(point_status);
      }
      None => {}
    }
  }

  // Increase
  for j in 0..BOARD_SIZE {
    let mut _j = j;
    let mut stones: Vec<PointStatus> = vec![];
    for i in 0..BOARD_SIZE {
      stones.push(board[i][_j].clone());
      if _j == 0 {
        break;
      }
      _j -= 1;
    }

    if stones.len() < 5 {
      continue;
    }

    match is_continuous_stone_exist(stones) {
      Some(point_status) => {
        return Some(point_status);
      }
      None => {}
    }
  }
  for i in 0..BOARD_SIZE {
    let mut _i = i;
    let mut stones: Vec<PointStatus> = vec![];
    for j in (0..BOARD_SIZE).rev() {
      stones.push(board[_i][j].clone());
      if _i == BOARD_SIZE - 1 {
        break;
      }
      _i += 1;
    }

    if stones.len() < 5 {
      continue;
    }

    match is_continuous_stone_exist(stones) {
      Some(point_status) => {
        return Some(point_status);
      }
      None => {}
    }
  }

  return None;
}

fn is_continuous_stone_exist(stones: Vec<PointStatus>) -> Option<PointStatus> {
  let mut count: i8 = 0;
  let mut status: PointStatus = PointStatus::Empty;
  for point in stones.iter() {
    match point {
      PointStatus::Empty => {
        count = 0;
        status = PointStatus::Empty;
      }
      _status => {
        if status != *_status {
          status = *_status;
          count = 1;
        } else {
          count += 1;
        }
      }
    }
    if count >= 5 {
      return Some(status);
    }
  }
  return None;
}
