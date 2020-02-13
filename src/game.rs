pub mod player;
use player::Player;

pub struct Game{
    pub my_player: Player,
}

impl Game{
    pub fn new() -> Game{
        Game{
            my_player:Player::new(),
        }
    }
    pub fn move_player(&mut self, _x:i32, _y:i32){
        self.my_player.move_player(_x, _y);
    }
}