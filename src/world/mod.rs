pub mod chunk;
pub mod world_properties;

use std::iter::Filter;
use std::collections::HashMap;
use std::io::{self, Read, Write};

use uuid::Uuid;

use self::chunk::{Chunk, ChunkPos, BlockID};
use self::world_properties::WorldProperties;
use math::Vec3;
use proto;
use binary;

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
    spawn_pos: BlockPos,
    properties: WorldProperties,
}

impl World {
    pub fn new(dimension: Dimension) -> World {
        World { chunks: HashMap::new(), properties: WorldProperties::new(dimension), spawn_pos: BlockPos::ZERO }
    }

    pub fn get_spawn_pos(&self) -> BlockPos {
        self.spawn_pos
    }

    pub fn get_block(&self, pos: BlockPos) -> Option<BlockID> {
        self.chunks.get(&ChunkPos::from(pos)).and_then(|c| {
            let (x, y, z) = pos.to_relative_chunk_pos();
            Some(c.get_block(x, y, z))
        })
    }

    /// Sets the block at given position.
    /// # Panics
    /// If the chunk is not loaded, it will panic.
    pub fn set_block(&mut self, pos: BlockPos, block: BlockID) {
        if let Some(c) = self.chunks.get_mut(&ChunkPos::from(pos)) {
            let (x, y, z) = pos.to_relative_chunk_pos();
            c.set_block(x, y, z, block);
        } else {
            panic!("tried setting block at pos ({}) when the chunk is not loaded");
        }
    }

    /// gets the chunk from the world, loaded if required, and adds player as who see.
    pub fn get_chunk(&mut self, pos: ChunkPos, player: Uuid) -> &mut Chunk {
        if !self.chunks.contains_key(&pos) {
            self.load_chunk(pos);
        }

        let chunk = self.chunks.get_mut(&pos).unwrap();
        chunk.insert_player_in_vicinity(player);
        chunk
    }

    /// unloads the chunk when it is abandoned. removes player as who see.
    /// Chunks in spawn (radius of 10) will never be unloaded.
    pub fn unload_chunk_if_required(&mut self, pos: ChunkPos, player: Uuid) {
        let spawn_rect = ChunkRectangle::centered(self.get_spawn_pos().into(), 10);
        let unload = if let Some(chk) = self.chunks.get_mut(&pos) {
            chk.remove_player_in_vicinity(&player);
            if chk.is_abandoned() && !spawn_rect.contains(pos) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockPos {
    pub x: i32,
    pub y: u8,
    pub z: i32,
}

impl BlockPos {
    pub const ZERO: BlockPos = BlockPos { x: 0, y: 0, z: 0 };

    pub fn new(x: i32, y: u8, z: i32) -> BlockPos {
        BlockPos { x, y, z }
    }

    pub fn write_proto<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut val: u64 = (self.z & 0x3ff_ffff) as u64;
        val |= (self.y as u64 & 0xfff) << 26;
        val |= (self.x as u64 & 0x3ff_ffff) << 38;
        binary::write_long(w, val as i64)
    }

    pub fn read_proto<R: Read>(r: &mut R) -> proto::Result<BlockPos> {
        let val = binary::read_long(r)?;
        let x = (val >> 38) as i32;
        let y = ((val << 26) >> 52) as u8;
        let z = (val as i32) << 6 >> 6;
        Ok(BlockPos { x, y, z })
    }

    pub fn to_relative_chunk_pos(&self) -> (u8, u8, u8) {
        let x = if self.x < 0 { 16 + self.x % 16 } else { self.x % 16 };
        let z = if self.z < 0 { 16 + self.z % 16 } else { self.z % 16 };
        (x as u8, self.y, z as u8)
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

    pub fn centered(center: ChunkPos, radius: u8) -> ChunkRectangle {
        ChunkRectangle::new(ChunkPos::new(center.x - radius as i32, center.z - radius as i32),
                            ChunkPos::new(center.x + radius as i32, center.z + radius as i32))
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