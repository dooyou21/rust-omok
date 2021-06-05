use rand::prelude::*;
use std::io::{self, Write};

const BOARD_SIZE: usize = 19;

enum Direction {
  Vertical,
  Horizontal,
  DiagonalLeftBottom,
  DiagonalRightBottom,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }
}

enum Color {
  Black,
  White,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PointStatus {
  Empty,
  Black(Point),
  White(Point),
}

impl PointStatus {
  fn print(&self) -> &str {
    match self {
      PointStatus::Empty => " |",
      PointStatus::Black(_) => "●|",
      PointStatus::White(_) => "○|",
    }
  }
}

enum Player {
  Player1, // computer
  Player2, // user
}

pub fn run() {
  let mut board = [[PointStatus::Empty; BOARD_SIZE]; BOARD_SIZE];
  let mut turn: Player = Player::Player1;
  let mut is_game_end: bool = false;
  let mut count = 10;

  print_board(&board);

  loop {
    if is_game_end {
      break;
    }

    match turn {
      Player::Player1 => {
        let next_point: Point = findPoint(Color::White, &board);
        place_stone(next_point.x, next_point.y, Color::White, &mut board);
        turn = Player::Player2;
      }
      Player::Player2 => {
        let next_point: Point = findPoint(Color::Black, &board);
        place_stone(next_point.x, next_point.y, Color::Black, &mut board);
        turn = Player::Player1;
        // TODO: Player2를 유저의 input을 받도록 한다.
      }
    }
    if count - 1 < 0 {
      is_game_end = true;
    } else {
      count -= 1;
    }
  }

  print_board(&board);
}

fn findPoint(color: Color, board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> Point {
  let mut x = rand::thread_rng().gen_range(0..19);
  let mut y = rand::thread_rng().gen_range(0..19);

  while board[x][y] != PointStatus::Empty {
    x = rand::thread_rng().gen_range(0..19);
    y = rand::thread_rng().gen_range(0..19);
  }

  Point::new(x, y)
}

fn place_stone(
  x: usize,
  y: usize,
  color: Color,
  board: &mut [[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) {
  // if (board[x][y] != PointStatus.Empty) 일때 에러처리 필요함.
  let point = Point::new(x, y);

  let point_status = match color {
    Color::White => PointStatus::White(point),
    Color::Black => PointStatus::Black(point),
  };
  board[x][y] = point_status;
}

fn print_board(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) {
  print!("| |0|1|2|3|4|5|6|7|8|9|0|1|2|3|4|5|6|7|8|");
  for (i, points) in board.iter().enumerate() {
    print!("\n|{}|", i % 10);
    for point in points.iter() {
      print!("{}", point.print());
    }
    io::stdout().flush().unwrap();
  }
  print!("\n\n");
  io::stdout().flush().unwrap();
}
