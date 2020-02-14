
//this pulls in everything declared in game.rs
mod game;
//Since we only really have game state, to avoid using "game::game_state::GameState" everywhere,
//we pull just the gamestate into the scope. 

//note that "mod game" gives us scope of "src/game/mod.rs" and declared there is a 
//"pub mod game_state" which lets us "use game::game_state"

//"use game::game_state" gives us access to everything in "src/game/game_state.rs"
//so we just go one step further and bring ONLY the "GameState" struct into scope
//using "use game::game_state::GameState"
use game::game_state::GameState;

fn main() {
    //since we `use game::GameState` above we can just create a new GameState
    //new is a function of GameState, check in the game.rs file
    let my_game = GameState::new();

    //this function contains the main game loop, main just goes into there,
    //and then doesn't come back here till the game is exited.
    my_game.main_loop();

    //just a message for when the program finally exits
    println!("Thanks for playing?")
}