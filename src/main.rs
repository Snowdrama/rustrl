// use std::io;
// use game::GameState;
mod game;

fn main() {
    game::player::hello();
    let my_player = game::player::Player::new();
    println!("Player is at {}, {}", my_player.x, my_player.y);
}