mod map_chunk;
use map_chunk::MapChunk;

pub struct Map{
    pub blocks: [[i32;256];256],
}

impl Map{
    pub fn new() -> Map{
        Map{
            blocks:[[0;256];256],
        }
    }
    pub fn GenerateOverworldMap(&mut self){
        //Start empty and add items
        // 
        // Start low rez and then blow each tile up to some scale
        // 
        // Start with something like this from a perlin heightmap
        // 0000000000
        // 1100011100
        // 2111112110
        // 2222221100
        // 
        // then designate some towns
        // 0000000000
        // 1100011100
        // 211111_110
        // 2_222_1100
        // 
        // 
        // _->Towns
        // then then make each of those are a 128x128 grid or something
        // height determines biome
        // 0->water
        // 1->beach
        // 2->forest
        // 3->mountain
        // towns are internally generated separately upon expansion to the large chunked grid
        // 
    }
}