use crate::game::{PointStatus, BOARD_SIZE};
use std::io::{self, Write};

pub fn print_board(board: &[[PointStatus; BOARD_SIZE]; BOARD_SIZE]) {
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
