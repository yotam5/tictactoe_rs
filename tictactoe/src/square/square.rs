use super::square_state::SquareState;

#[derive(Copy,Clone, Eq,PartialEq)]
pub struct Square(SquareState);

impl Square
{
    pub fn new(square_state: SquareState) -> Self
    {
        Square(square_state)
    }

    pub fn get_display_char(&self) -> char
    {
        self.0.get_dispaly_char()
    }

    pub fn get_state(&self) -> SquareState{
        self.0
    }
}

impl Default for Square
{
    fn default() -> Self {
       Square::new(SquareState::Empty)
    }
}
