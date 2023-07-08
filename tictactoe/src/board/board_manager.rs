use super::board::Board;
use crate::{constants::constants::{DEFAULT_PLAYER, BOARD_SIDE}, square::square_state::SquareState};
use super::board_analyzer::analyze_grid_state;
type MyResult = Result<(), &'static str>;

#[derive(Copy,Clone, PartialEq, Eq, Debug)]
pub enum GameState
{
    Winner(Player),
    Tie,
    InProgress
}

#[derive(Clone, Copy,PartialEq, Eq, Debug)]
pub enum Player{
    PlayerX,
    PlayerO,
}

impl Into<SquareState> for Player
{
    fn into(self) -> SquareState {
        match self 
        {
            Player::PlayerX => 
            {
                SquareState::X
            },
            Player::PlayerO =>
            {
                SquareState::O
            }
        }
    }
}

pub struct BoardManager
{
    board:Board,
    current_player: Player,
    game_state: GameState,
    game_turn: usize
}

impl Default for BoardManager
{
    fn default() -> Self {
        BoardManager { board: Board::default(), current_player: DEFAULT_PLAYER,game_state: GameState::InProgress, game_turn: 0 }
    }

}

impl BoardManager
{   
    
    pub fn make_move(&mut self, row_idx: usize, column_idx: usize) -> MyResult
    {
        if self.game_turn == BOARD_SIDE * BOARD_SIDE
        {
            self.game_state = GameState::Tie;
        }

        self.is_move_valid(row_idx, column_idx)?;
        self.board.set_value_at_index(row_idx, column_idx, self.current_player.into())?;
        self.change_player();
        self.game_state = analyze_grid_state(self.board.get_grid());
        self.game_turn += 1;
        

        Ok(())
    }

    pub fn change_player(&mut self)
    {
        self.current_player = match self.current_player {
            Player::PlayerO => Player::PlayerX,
            Player::PlayerX => Player::PlayerO,
        }
    }

    pub fn get_game_state(&self) -> GameState
    {
        self.game_state
    }

    pub fn is_move_valid(&self, row_idx: usize, column_idx: usize) -> MyResult
    {
        let valid_idx = self.board.get_value_at_index(row_idx, column_idx);
        
        match valid_idx{
           Some(value)  => 
           {
               if value.get_state() != SquareState::Empty{
                   return Err("Index In Use");
               }
           },
           None => {
               return Err("Invalid Index");
           }
        }

        Ok(())
    }

    pub fn get_board(&self) -> &Board
    {
        &self.board
    }

}
