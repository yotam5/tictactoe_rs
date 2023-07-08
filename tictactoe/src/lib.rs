
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