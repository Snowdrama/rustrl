pub mod player;
pub mod map;

use player::Player;
use map::Map;

pub struct Game{
    pub my_player: Player,
    pub world_map: Map,
}

impl Game{
    pub fn new() -> Game{
        Game{
            my_player:Player::new(),
            world_map: Map::new(),
        }
    }
    pub fn move_player(&mut self, _x:i32, _y:i32){
        self.my_player.move_player(_x, _y);
    }
}