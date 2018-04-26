pub mod chunk;
pub mod world_properties;

use std::iter::Filter;
use std::collections::HashMap;

use uuid::Uuid;

use self::chunk::{Chunk, ChunkPos};
use self::world_properties::WorldProperties;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelType {
    Default,
    Flat,
    LargeBiomes,
    Amplified,
    Default11,
}

impl LevelType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            LevelType::Default => "default",
            LevelType::Flat => "flat",
            LevelType::LargeBiomes => "largeBiomes",
            LevelType::Amplified => "amplified",
            LevelType::Default11 => "default_1_1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    Nether = -1,
    Overworld,
    End,
}

pub struct World {
    chunks: HashMap<ChunkPos, Chunk>,
    properties: WorldProperties,
}

impl World {
    pub fn new(dimension: Dimension) -> World {
        World { chunks: HashMap::new(), properties: WorldProperties::new(dimension) }
    }

    /// gets the chunk from the world, loaded if required, and adds player as who see.
    pub fn get_chunk(&mut self, pos: ChunkPos, player: Uuid) -> &mut Chunk {
        if !self.chunks.contains_key(&pos) {
            self.load_chunk(pos);
        }

        let chunk = self.chunks.get_mut(&pos).unwrap();
        chunk.insert_player_who_see(player);
        chunk
    }

    /// unloads the chunk when it is abandoned. removes player as who see.
    pub fn unload_chunk_if_required(&mut self, pos: ChunkPos, player: Uuid) {
        let unload = if let Some(chk) = self.chunks.get_mut(&pos) {
            chk.remove_player_who_see(&player);
            if chk.is_abandoned() {
                true
            } else {
                false
            }
        } else { false };

        if unload {
            self.chunks.remove(&pos);
        }
    }

    fn load_chunk(&mut self, pos: ChunkPos) {
        println!("chunk load");
        let chk = self.properties.load_chunk(pos);
        self.chunks.insert(pos, chk);
    }

    pub fn get_properties(&self) -> &WorldProperties {
        &self.properties
    }
}

/// Rectangle of chunks in view-distance of player.
/// ```
///              N
///              ^ -z
///              |
///              |
/// W -x <-------|-------> x E
///              |
///              |
///              ; z
///              N
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkRectangle {
    min: ChunkPos,
    max: ChunkPos,
}

// min is (minx, minz). max is (maxx, maxz)
impl ChunkRectangle {
    pub fn new(pos1: ChunkPos, pos2: ChunkPos) -> ChunkRectangle {
        ChunkRectangle {
            min: ChunkPos::new(i32::min(pos1.x, pos2.x), i32::min(pos1.z, pos2.z)),
            max: ChunkPos::new(i32::max(pos1.x, pos2.x), i32::max(pos1.z, pos2.z)),
        }
    }

    pub fn contains(&self, pos: ChunkPos) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x &&
            pos.z >= self.min.z && pos.z <= self.max.z
    }

    pub fn chunks_iter(&self) -> ChunksIter {
        ChunksIter::new(*self)
    }

    /// Return an iterator of chunk pos present in self, but not `other`
    pub fn subtract_iter(self, other: ChunkRectangle) -> impl Iterator<Item=ChunkPos> {
        self.chunks_iter().filter(move |&p| !other.contains(p))
    }
}

pub struct ChunksIter {
    rect: ChunkRectangle,
    x: i32,
    z: i32,
}

impl ChunksIter {
    fn new(rect: ChunkRectangle) -> ChunksIter {
        ChunksIter { rect, x: rect.min.x, z: rect.min.z }
    }
}

impl Iterator for ChunksIter {
    type Item = ChunkPos;

    fn next(&mut self) -> Option<Self::Item> {
        /*for z in self.rect.min.z..(self.rect.max.z + 1) {
            for x in self.rect.min.x..(self.rect.max.x + 1) {
                yield return ChunkPos::new(x, z);
            }
        }*/

        if self.z > self.rect.max.z {
            return None;
        }

        let ret = ChunkPos::new(self.x, self.z);
        self.x += 1;
        if self.x > self.rect.max.x {
            self.z += 1;
            self.x = self.rect.min.x;
        }

        Some(ret)
    }
}