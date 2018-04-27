use noise::{Perlin, NoiseFn};

use world::chunk::{ChunkPos, Chunk, BlockID};
use world::Dimension;

/// perlin adjusted for range 0 to 1 instead of -1 to 1
fn perlin(x: f64, z: f64) -> f64 {
    let perlin = Perlin::new();
    (perlin.get([x, z]) + 1.0) / 2.0
}

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

        let perlin = Perlin::new();
        for z in 0..16 {
            for x in 0..16 {
                let (world_x, world_z) = ((pos.x * 16 + x as i32) as f64, (pos.z * 16 + z as i32) as f64);
                let p = self::perlin(world_x / 100.0, world_z / 100.0) * 20.0 +
                    self::perlin(world_x / 30.0 + 10.0, world_z / 20.0 + 50.0) * 5.0;
                let y = p as u8 + 1;
                for y in 1..=y {
                    chunk.set_block(x, y, z, BlockID::new(2, 0));
                }
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