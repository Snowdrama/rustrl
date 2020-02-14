pub struct Player{
    pub x: i32,
    pub y: i32,
}

// pub fn hello(){
//     println!("Hello from Player!");
// }

impl Player{
    //constructor
    pub fn new() -> Player{
        Player{
            x:0,
            y:0
        }
    }
    pub fn move_player(&mut self, x:i32, y:i32){
        self.x += x;
        self.y += y;
    }
}