use world::chunk::{ChunkPos, Chunk, BlockID};
use world::Dimension;
use std::time::Instant;

pub struct WorldProperties {
    dimension: Dimension,
}

impl WorldProperties {
    pub fn new(dimension: Dimension) -> WorldProperties {
        WorldProperties { dimension }
    }

    pub fn has_sky_light(&self) -> bool {
        match self.dimension {
            Dimension::Overworld => true,
            _ => false,
        }
    }

    pub fn load_chunk(&self, pos: ChunkPos) -> Chunk {
        match self.dimension {
            Dimension::Overworld => self.load_overworld(pos),
            Dimension::Nether => self.load_nether(pos),
            Dimension::End => self.load_end(pos),
        }
    }

    fn load_overworld(&self, pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(pos, true);

        for z in 0..16 {
            for x in 0..16 {
                chunk.set_block(x, 0, z, BlockID::new(7, 0));
            }
        }

        for y in 1..(79 + 1) {
            for z in 0..16 {
                for x in 0..16 {
                    chunk.set_block(x, y, z, BlockID::new(1, 0));
                }
            }
        }

        for z in 0..16 {
            for x in 0..16 {
                chunk.set_block(x, 80, z, BlockID::new(2, 0));
            }
        }

        chunk
    }

    fn load_nether(&self, pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(pos, false);

        for z in 0..16 {
            for x in 0..16 {
                chunk.set_block(x, 0, z, BlockID::new(7, 0));
            }
        }

        for y in 1..(80 + 1) {
            for z in 0..16 {
                for x in 0..16 {
                    chunk.set_block(x, y, z, BlockID::new(87, 0));
                }
            }
        }

        chunk
    }

    fn load_end(&self, pos: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(pos, false);

        for z in 0..16 {
            for x in 0..16 {
                chunk.set_block(x, 0, z, BlockID::new(7, 0));
            }
        }

        for y in 1..(80 + 1) {
            for z in 0..16 {
                for x in 0..16 {
                    chunk.set_block(x, y, z, BlockID::new(121, 0));
                }
            }
        }

        chunk
    }
}