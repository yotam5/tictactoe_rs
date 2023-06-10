
use crate::constants::constants::{BOARD_SIDE};
use crate::square::square::{Square};
use crate::square::square_state::{SquareState};

type MyResult = Result<(), &'static str>;
pub type Grid = [[Square;BOARD_SIDE ]; BOARD_SIDE];

use std::fmt;

pub struct Board
{
    grid: Grid,
}


impl Board
{
    pub fn new() -> Self{
        Board{grid: [[Square::default(); BOARD_SIDE]; BOARD_SIDE]}
    }

    pub fn is_valid_index(row_idx: usize, column_idx: usize) -> bool
    {
        row_idx < BOARD_SIDE && column_idx < BOARD_SIDE
    }

    pub fn get_value_at_index(&self, row_idx: usize, column_idx: usize) -> Option<Square>
    {
        let valid = Board::is_valid_index(row_idx, column_idx);
        
        if !valid {
            return None;
        }

        Some(self.grid[row_idx][column_idx])
    }

    pub fn set_value_at_index(&mut self, row_idx: usize, column_idx: usize, square_state: SquareState) -> MyResult
    {
        if !Board::is_valid_index(row_idx, column_idx)  {
            return Err("Out Of Index");
        }
        self.grid[row_idx][column_idx] = Square::new(square_state);
        Ok(())
    }

    pub fn get_grid(&self) -> &Grid
    {
        &self.grid
    }

}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       let mut cyclic_result_buff = [u8::default(); BOARD_SIDE];

       for (_, row) in self.grid.iter().enumerate(){
           for (column_idx, element) in row.iter().enumerate(){
                cyclic_result_buff[column_idx] = element.get_display_char() as u8;
           }
           let row_to_display = std::str::from_utf8(&cyclic_result_buff).unwrap();
           writeln!(f, "{}",row_to_display)?;

       }

       Ok(())
    }
}

impl Default for Board
{
    fn default() -> Self {
        Board::new()
    }
}
