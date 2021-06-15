use std::io::{self, Write};
mod calculator;
mod renderer;
mod tools;

pub const BOARD_SIZE: usize = 19;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }
}

#[derive(Debug)]
pub enum Color {
  Black,
  White,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointStatus {
  Empty,
  Black,
  White,
}

impl PointStatus {
  fn print(&self) -> &str {
    match self {
      PointStatus::Empty => " |",
      PointStatus::Black => "●|",
      PointStatus::White => "○|",
    }
  }
}

enum Player {
  Player1, // computer
  Player2, // user
}

pub fn run() {
  let mut board = [[PointStatus::Empty; BOARD_SIZE]; BOARD_SIZE];
  let mut turn: Player;
  let mut is_game_end: bool = false;
  let mut just_before_point: Point = Point::new(0, 0);

  print!("\nChoose your stone color - (b)lack / (w)hite : ");
  io::stdout().flush().unwrap();

  let player_colors = get_color_from_user(); // [computer, user];
  println!(
    "user:{:?}, computer:{:?}",
    player_colors[1], player_colors[0]
  );

  match player_colors[0] {
    Color::Black => {
      turn = Player::Player1;
    }
    Color::White => {
      turn = Player::Player2;
    }
  };

  loop {
    if is_game_end {
      break;
    }

    match turn {
      Player::Player1 => {
        // computer
        let next_point: Point = calculator::find_next_point(&just_before_point, &board);
        place_stone(next_point.x, next_point.y, &player_colors[0], &mut board);
        renderer::print_board(&board);
        turn = Player::Player2;
      }
      Player::Player2 => {
        // user
        let next_point = get_position_from_user();

        if next_point.x > 18 || next_point.y > 18 {
          println!("0 ~ 18 사이의 숫자를 입력해야 함!");
          continue;
        }

        if board[next_point.x][next_point.y] != PointStatus::Empty {
          println!("이미 돌이 있어서 둘 수 없음!");
          continue;
        }

        just_before_point = Point::new(next_point.x, next_point.y);
        place_stone(next_point.x, next_point.y, &player_colors[1], &mut board);
        renderer::print_board(&board);
        turn = Player::Player1;
      }
    }

    is_game_end = tools::check_game_end(&board);
  }

  renderer::print_board(&board);
}

fn place_stone(
  x: usize,
  y: usize,
  color: &Color,
  board: &mut [[PointStatus; BOARD_SIZE]; BOARD_SIZE],
) {
  // if (board[x][y] != PointStatus.Empty) 일때 에러처리 필요함.
  let point_status = match color {
    Color::White => PointStatus::White,
    Color::Black => PointStatus::Black,
  };
  board[x][y] = point_status;
}

fn get_color_from_user() -> [Color; 2] {
  let mut player_colors = [Color::Black, Color::White];
  let mut input_line = String::new();
  loop {
    io::stdin().read_line(&mut input_line).unwrap();
    let input = input_line.trim();
    if input.eq("b") || input.eq("B") {
      player_colors[1] = Color::Black;
      player_colors[0] = Color::White;
      break;
    } else if input.eq("w") || input.eq("W") {
      player_colors[1] = Color::White;
      player_colors[0] = Color::Black;
      break;
    } else {
      print!("enter (b) or (w)");
      io::stdout().flush().unwrap();
      continue;
    }
  }
  player_colors
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
  Point::new(y, x)
}
