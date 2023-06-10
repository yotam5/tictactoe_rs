use tictactoe::{board_manager::{BoardManager, GameState}, clear_screen, handle_input};


fn main() {
    let mut board_manager = BoardManager::default();


    while !matches!(board_manager.get_game_state(), GameState::Tie | GameState::Winner(_))
    {
        board_manager.show_board();
        
        let m = handle_input();

        board_manager.make_move(m.row, m.column).unwrap();

        clear_screen();

    }

    println!("Game Over: {:?}", board_manager.get_game_state());


}
