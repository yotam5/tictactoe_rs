#![no_std]
pub mod constants;
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



#[cfg(test)]
mod tests
{
    use crate::board::board::Board;
    use crate::board_manager::GameState;
    use crate::square::square_state::SquareState;
    use crate::board::board_analyzer::analyze_grid_state;
    use crate::board_manager::Player;

    #[test]
    fn test_vertical()
    {
        let mut  g = Board::default();
        g.set_value_at_index(0, 0, SquareState::X).unwrap();
        g.set_value_at_index(1, 0, SquareState::X).unwrap();
        g.set_value_at_index(2, 0,  SquareState::X).unwrap();
        
        let res = analyze_grid_state(g.get_grid());
        assert_eq!(res, GameState::Winner(Player::PlayerX));
    }


    #[test]
    fn test_horizontal()
    {
        let mut  g = Board::default();
        g.set_value_at_index(0, 0, SquareState::X).unwrap();
        g.set_value_at_index(0, 1, SquareState::X).unwrap();
        g.set_value_at_index(0, 2,  SquareState::X).unwrap();
        
        let res = analyze_grid_state(g.get_grid());
        assert_eq!(res, GameState::Winner(Player::PlayerX));
    }

    #[test]
    fn test_diagnol()
    {
        let mut  g = Board::default();
        g.set_value_at_index(0, 0, SquareState::X).unwrap();
        g.set_value_at_index(1, 1, SquareState::X).unwrap();
        g.set_value_at_index(2, 2,  SquareState::X).unwrap();
        
        let res = analyze_grid_state(g.get_grid());
        assert_eq!(res, GameState::Winner(Player::PlayerX));
    }

}