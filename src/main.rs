// use std::io;
// use game::GameState;
mod game;

fn main() {
    game::player::hello();
    let mut game_state = game::Game::new();
    game_state.move_player(1,1);
    println!("Player is at {}, {}", game_state.my_player.x, game_state.my_player.y);
    game_state.move_player(1,1);
    println!("Player is at {}, {}", game_state.my_player.x, game_state.my_player.y);
    game_state.move_player(1,1);
    println!("Player is at {}, {}", game_state.my_player.x, game_state.my_player.y);
}