//So this is a bit confusing but let's hammer it out.
//the "player.rs" exists in the same folder as "game_state.rs"
//both of these are modules in themselves. So we've declared them
//as part of the "game" module in "src/game/mod.rs".

//to use the player struct, we need to bring Player into this scope.
//To do this we go through the game module using "super" which gives us
//access to our parent scope "game" from there we use "player::Player"
//through "super" to get to player.
use super::player::Player;

//This is the GameState struct
//Rust forces you to separate data from implementation
//So objects are just data, and then we use functions to 
//opperate on that data
pub struct GameState{
    pub my_player: Player,
}

//This block here is the functions that belong to GameState
impl GameState{
    //technically rust has no "constructor" in a traditional sense
    //so we just define a "new" function of GameState that returns 
    //an instantiated GameState object. 
    pub fn new() -> GameState{
        // -> GameState
        GameState{
            my_player:Player::new(),
        }
    }

    //we loop here the game note the "&mut self"
    //this means that the function belongs to the 
    //instance of the object itself. Look at main.rs
    //for the use of "GameState::new" which doesn't
    //have a "&mut self" so it's called statically
    //where this is used by "my_game_state.main_loop()"
    //because it's passing itself as a mutable reference 
    //to the function(so we can modify the state) 
    pub fn main_loop(&mut self){

    }
}