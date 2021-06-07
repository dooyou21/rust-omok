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

#[derive(Debug)]
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

  print!("\nChoose your stone color - (b)lack / (w)hite : ");
  io::stdout().flush().unwrap();

  let mut input_line = String::new();
  let mut player_colors = [Color::Black, Color::White]; // [computer, user];

  loop {
    io::stdin().read_line(&mut input_line).unwrap();
    let input = input_line.trim();
    if input.eq("b") || input.eq("B") {
      player_colors[1] = Color::Black;
      player_colors[0] = Color::White;
      turn = Player::Player2;
      break;
    } else if input.eq("w") || input.eq("W") {
      player_colors[1] = Color::White;
      player_colors[0] = Color::Black;
      turn = Player::Player1;
      break;
    } else {
      print!("enter (b) or (w)");
      io::stdout().flush().unwrap();
      continue;
    }
  }

  print_board(&board);
  println!(
    "user:{:?}, computer:{:?}",
    player_colors[1], player_colors[0]
  );

  loop {
    if is_game_end {
      break;
    }

    match turn {
      Player::Player1 => {
        // computer
        let next_point: Point = find_point(&player_colors[0], &board);
        place_stone(next_point.x, next_point.y, &player_colors[0], &mut board);
        print_board(&board);
        turn = Player::Player2;
      }
      Player::Player2 => {
        // user
        let next_point = get_position_from_user();
        place_stone(next_point.x, next_point.y, &player_colors[1], &mut board);
        print_board(&board);
        turn = Player::Player1;
      }
    }

    is_game_end = check_game_end(&board);
  }

  print_board(&board);
}

fn find_point(color: &Color, board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> Point {
  let mut x = rand::thread_rng().gen_range(0..19);
  let mut y = rand::thread_rng().gen_range(0..19);

  while board[x][y] != PointStatus::Empty {
    x = rand::thread_rng().gen_range(0..19);
    y = rand::thread_rng().gen_range(0..19);
  }

  Point::new(x, y)
}

fn check_game_end(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) -> bool {
  // TODO: 게임이 끝났는지 판단할 수 있어야 함.
  false
}

fn place_stone(
  x: usize,
  y: usize,
  color: &Color,
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

fn get_position_from_user() -> Point {
  print!("x: ");
  io::stdout().flush().unwrap();
  let mut x_input = String::new();
  io::stdin().read_line(&mut x_input).unwrap();
  let x = x_input.trim().parse::<usize>().unwrap();
  print!("y: ");
  io::stdout().flush().unwrap();
  let mut y_input = String::new();
  io::stdin().read_line(&mut y_input).unwrap();
  let y = y_input.trim().parse::<usize>().unwrap();
  Point::new(x, y)
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
