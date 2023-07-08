
use crate::constants::constants::{X_DISPLAY, O_DISPLAY, EMPTY_DISPLAY};

//SquareState
#[derive(Copy,Clone,PartialEq, Eq, Debug)]
pub enum SquareState{
    X,
    O,
    Empty
}

impl SquareState
{
    pub fn get_dispaly_char(&self) -> char{
        match self {
            SquareState::X => X_DISPLAY,
            SquareState::O => O_DISPLAY,
            SquareState::Empty => EMPTY_DISPLAY
        }
    }
}
