pub mod chunk;
pub mod chunk_provider;

use std::collections::HashMap;

use uuid::Uuid;

use self::chunk::{Chunk, ChunkPos};
use self::chunk_provider::ChunkProvider;

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

pub enum Dimension {
    Nether = -1,
    Overworld,
    End,
}

pub struct World {
    name: String,
    uuid: Uuid,
    chunks: HashMap<ChunkPos, Chunk>,
    dimension: Dimension,
    chunk_provider: ChunkProvider,
}

impl World {
    pub fn new(name: String, uuid: Uuid) -> World {
        World { name, uuid, chunks: HashMap::new(), dimension: Dimension::Overworld, chunk_provider: ChunkProvider }
    }
}