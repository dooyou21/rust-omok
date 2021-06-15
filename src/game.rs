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

#[derive(Debug, PartialEq)]
pub enum Color {
  Black,
  White,
}

impl Color {
  fn get_point_status(&self) -> PointStatus {
    match self {
      Color::White => PointStatus::White,
      Color::Black => PointStatus::Black,
    }
  }
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
  fn get_color(&self) -> Option<Color> {
    match self {
      PointStatus::Empty => None,
      PointStatus::Black => Some(Color::Black),
      PointStatus::White => Some(Color::White),
    }
  }
}

enum Player {
  Player1, // computer
  Player2, // user
}

pub struct Board {
  board: [[PointStatus; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
  fn new() -> Board {
    Board {
      board: [[PointStatus::Empty; BOARD_SIZE]; BOARD_SIZE],
    }
  }
  fn check(&self, point: &Point) -> PointStatus {
    self.board[point.y][point.x]
  }
  fn place_stone(&mut self, point: Point, color: &Color) -> Result<Point, &str> {
    if self.board[point.y][point.x] != PointStatus::Empty {
      return Err("Already exist");
    } else {
      self.board[point.y][point.x] = color.get_point_status();
      return Ok(point);
    }
  }
  fn print(&self) {
    renderer::print_board(&self.board);
  }
  fn is_game_end(&self) -> Option<Color> {
    match tools::check_game_end(&self.board) {
      None => None,
      Some(_status) => _status.get_color(),
    }
  }
}

pub fn run() {
  let mut board = Board::new();
  let mut turn: Player;
  let mut just_before_point: Point = Point::new(0, 0);

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
    match board.is_game_end() {
      Some(_color) => {
        if _color == player_colors[0] {
          println!("====You Lose.. Blue win!====");
        } else {
          println!("====You win!====");
        }
        board.print();
        break;
      }
      None => {}
    }

    match turn {
      Player::Player1 => {
        // computer
        let next_point = calculator::find_next_point(&just_before_point, &player_colors[0], &board);
        match board.place_stone(next_point, &player_colors[0]) {
          Err(error_message) => {
            println!("{}", error_message);
            continue;
          }
          Ok(point) => {
            println!("blue: {:?}", point);
            just_before_point = point;
            turn = Player::Player2;
            board.print();
          }
        }
      }
      Player::Player2 => {
        // user
        let next_point = get_position_from_user();
        match board.place_stone(next_point, &player_colors[1]) {
          Err(error_message) => {
            println!("{}", error_message);
            continue;
          }
          Ok(point) => {
            just_before_point = point;
            turn = Player::Player1;
            board.print();
          }
        }
      }
    }
  }
}

fn get_color_from_user() -> [Color; 2] {
  let mut player_colors = [Color::Black, Color::White];
  loop {
    print!("\nChoose your stone color - (b)lack / (w)hite : ");
    io::stdout().flush().unwrap();
    let mut input_line = String::new();
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
      println!("[ERROR]: Unrecognized character");
      continue;
    }
  }
  player_colors
}

fn get_position_from_user() -> Point {
  let x: usize;
  let y: usize;
  loop {
    print!("x: ");
    io::stdout().flush().unwrap();
    let mut x_input = String::new();
    io::stdin().read_line(&mut x_input).unwrap();
    match x_input.trim().parse::<usize>() {
      Ok(_x) => {
        if _x >= BOARD_SIZE {
          println!("[ERROR]: Out of range [0 ~ 19)");
          continue;
        }
        x = _x;
        break;
      }
      Err(_) => {
        println!("[ERROR]: Not a number");
        continue;
      }
    }
  }
  loop {
    print!("y: ");
    io::stdout().flush().unwrap();
    let mut y_input = String::new();
    io::stdin().read_line(&mut y_input).unwrap();
    match y_input.trim().parse::<usize>() {
      Ok(_y) => {
        if _y >= BOARD_SIZE {
          println!("[ERROR]: Out of range [0 ~ 19)");
          continue;
        }
        y = _y;
        break;
      }
      Err(_) => {
        println!("[ERROR]: Not a number");
        continue;
      }
    }
  }
  Point::new(x, y)
}
