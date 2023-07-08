use std::io::{stdin, stdout, Write};
use tictactoe_llem::{board_manager::{BoardManager, GameState}, Move, constants::constants::BOARD_SIDE};

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

fn main() {
    let mut board_manager = BoardManager::default();


    while !matches!(board_manager.get_game_state(), GameState::Tie | GameState::Winner(_))
    {
        println!("{}", board_manager.get_board());     
        let m = handle_input();

        board_manager.make_move(m.row, m.column).unwrap();

        clear_screen();

    }

    println!("Game Over: {:?}", board_manager.get_game_state());


}
