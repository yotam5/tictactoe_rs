
use std::io::{stdin, stdout, Write};
use crate::constants::constants::BOARD_SIDE;
mod constants;
mod square;
mod board;
pub use board::board_manager;


pub struct Move
{
    pub row: usize,
    pub column: usize,
}

impl Move
{
    pub fn new(row: usize, column: usize) -> Self
    {
        Move{row, column}
    }
}

pub fn handle_input() -> Move {
    let mut input = String::new();
    print!("enter row ({}-{}): ", 0, BOARD_SIDE);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("error reading input");
    let mut row = input.trim().parse::<usize>().expect("error parsing input");

    input.clear();
    print!("enter column ({}-{}): ", 0, BOARD_SIDE);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("error reading input");
    let mut col = input.trim().parse::<usize>().expect("error parsing input");

    col -= 1;
    row -= 1;

    Move{row: row, column: col}

}

/// Clear the terminal display
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}