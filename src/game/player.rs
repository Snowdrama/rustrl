pub struct Player{
    pub x: i32,
    pub y: i32,
}

pub fn hello(){
    println!("Hello from Player!");
}

impl Player{
    pub fn new() -> Player{
        Player{
            x:0,
            y:0
        }
    }
}