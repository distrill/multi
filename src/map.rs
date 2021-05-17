use tetra::Context;

use serde::{Serialize, Deserialize};

use crate::chunk::Chunk;
use crate::constants::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub chunks: Vec<Vec<Chunk>>,
}

impl Map {
    pub fn new() -> Map {
        let mut chunks = Vec::with_capacity(MAP_CHUNK_WIDTH);
        for i in 0..MAP_CHUNK_WIDTH as i32 {
            let mut col  = Vec::with_capacity(MAP_CHUNK_HEIGHT);
            for j in 0..MAP_CHUNK_HEIGHT as i32 {
                col.push(Chunk::new(i, j));
            }
            chunks.push(col);
        }
        
        Map { chunks }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        for col in self.chunks.iter_mut() {
            for chunk in col.iter_mut() {
                chunk.draw(ctx)?;
            }
        }
        Ok(())
    }
}
