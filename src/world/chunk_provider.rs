use world::chunk::{ChunkPos, Chunk, BlockID};
use std::time::Instant;

pub struct ChunkProvider;

impl ChunkProvider {
    pub fn load_chunk(pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(pos, true);
        for y in 0..80 {
            for z in 0..16 {
                for x in 0..16 {
                    chunk.set_block(x, y, z, BlockID::new(y, 0));
                }
            }
        }

        chunk
    }
}