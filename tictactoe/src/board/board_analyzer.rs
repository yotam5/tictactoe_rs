use super::board::{Grid};
use crate::constants::constants::BOARD_SIDE;
use crate::square::square::Square;
use crate::square::square_state::SquareState;
use crate::board_manager::{GameState, Player};


pub fn analyze_grid_state(grid: &Grid) -> GameState
{
    let horz = check_win_horizontally(grid);
    let vert = check_win_vertically(grid);
    let diag = check_win_diagnolly(grid);

    let wins = [horz, vert, diag];

    for w in wins
    {
        if matches!(w, GameState::Winner(_)){
            return  w;
        }
    }

    GameState::InProgress

}

fn check_win_horizontally(grid: &Grid) -> GameState
{
    for (_,row) in grid.iter().enumerate()
    {
        let mut counter_x = 0;
        let mut counter_o = 0;

        for value in row
        {
            match value.get_state()
            {
                SquareState::O => {counter_o +=1; },
                SquareState::X => {counter_x += 1;},
                _ => {}
            }
        }
        if counter_x == BOARD_SIDE{
            return GameState::Winner(Player::PlayerX);
        }
        if counter_o == BOARD_SIDE
        {
            return GameState::Winner(Player::PlayerO);
        }
    }

    GameState::InProgress

}

fn check_win_vertically(grid: &Grid) -> GameState {

    for column in 0..BOARD_SIDE {
        let mut counter: i32 = 0;
        
        for row in 0..BOARD_SIDE {
            match grid[row][column].get_state() {
                SquareState::X => counter += 1,
                SquareState::O => counter -= 1,
                _ => {}
            }
        }
        if counter == BOARD_SIDE as i32{
            return GameState::Winner(Player::PlayerX);
        }
        if counter == -(BOARD_SIDE as i32)
        {
            return GameState::Winner(Player::PlayerO);
        }
    }


    GameState::InProgress
}

fn check_win_diagnolly(grid: &Grid) -> GameState 
{

    let mut counter_bt= 0;
    let mut counter_tb = 0;

    for i in 0..BOARD_SIDE
    {
        let square_bt: Square = grid[i][i];
        let square_tb: Square = grid[i][i];

        match square_bt.get_state()
        {
            SquareState::O => {counter_bt += 1;},
            SquareState::X => {counter_bt-= 1;},
            _ => {}
        }

        match square_tb.get_state()
        {
            SquareState::O => {counter_tb += 1;},
            SquareState::X => {counter_tb-= 1;},
            _ => {}
        }
    }

    if counter_bt == BOARD_SIDE as i32|| counter_tb == BOARD_SIDE as i32
    {
        return GameState::Winner(Player::PlayerO);
    }
    if counter_bt == -(BOARD_SIDE as i32) || counter_tb == -(BOARD_SIDE as i32)
    {
        return GameState::Winner(Player::PlayerX);
    }
    
    GameState::InProgress



}
