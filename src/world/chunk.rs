use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::block::block_entity::BlockEntity;
use crate::block::{Block, BlockPos, BlockStateId};
use crate::collections::{NibbleArray, VarbitArray};
use crate::math::Vec3;
use crate::proto::data::{self, GroundUpContinuous, GroundUpNonContinuous};
use crate::proto::packets::{SPacket, SPlayChunkDataData, SPlayMapChunkBulkData};

pub const CHUNK_SECTION_BLOCK_COUNT: usize = 16 * 16 * 16;

#[derive(Debug)]
pub struct ChunkSection {
    // this is not used in 1.8 but saves memory as much as 2x, sacrificing performance a bit.
    palette: Vec<BlockStateId>,
    blocks: VarbitArray,
    block_light: NibbleArray,
    sky_light: Option<NibbleArray>,
    air_count: u16,
}

impl ChunkSection {
    pub fn new(has_sky_light: bool) -> ChunkSection {
        ChunkSection {
            palette: vec![BlockStateId::AIR],
            blocks: VarbitArray::new(4, CHUNK_SECTION_BLOCK_COUNT),
            block_light: NibbleArray::new_with_default(CHUNK_SECTION_BLOCK_COUNT, 15),
            sky_light: if has_sky_light {
                Some(NibbleArray::new_with_default(CHUNK_SECTION_BLOCK_COUNT, 15))
            } else {
                None
            },
            air_count: CHUNK_SECTION_BLOCK_COUNT as u16,
        }
    }

    fn get_linear_index(x: u8, y: u8, z: u8) -> usize {
        (y & 0x0f) as usize * 16 * 16 + (z & 0x0f) as usize * 16 + (x & 0x0f) as usize
    }

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> BlockStateId {
        self.palette[self.blocks.get(Self::get_linear_index(x, y, z)) as usize]
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, b: BlockStateId) {
        let previous = self.get_block(x, y, z);
        if previous == b {
            return;
        }

        if previous.get_type() == Block::Air {
            self.air_count -= 1;
        } else if b.get_type() == Block::Air {
            self.air_count += 1;
        }

        if let Some(index) = self.palette.iter().position(|&e| e == b) {
            let pos = Self::get_linear_index(x, y, z);
            self.blocks.set(pos, index as u64);
        } else {
            self.palette.push(b);

            let needed_bit_size = VarbitArray::bit_size_needed(self.palette.len());
            let current_bit_size = self.blocks.bit_size();

            if needed_bit_size > current_bit_size {
                self.blocks.change_bit_size(needed_bit_size);
            }

            self.blocks.set(
                Self::get_linear_index(x, y, z),
                (self.palette.len() - 1) as u64,
            );
        }
    }

    pub fn is_empty(&self) -> bool {
        self.air_count == CHUNK_SECTION_BLOCK_COUNT as u16
    }

    pub fn set_block_light(&mut self, x: u8, y: u8, z: u8, light: u8) {
        self.block_light.set(Self::get_linear_index(x, y, z), light);
    }

    pub fn get_block_light(&self, x: u8, y: u8, z: u8) -> u8 {
        self.block_light.get(Self::get_linear_index(x, y, z))
    }

    pub fn set_sky_light(&mut self, x: u8, y: u8, z: u8, light: u8) {
        if let Some(ref mut s) = self.sky_light {
            s.set(Self::get_linear_index(x, y, z), light);
        }
    }

    pub fn get_sky_light(&self, x: u8, y: u8, z: u8) -> u8 {
        match self.sky_light {
            Some(ref s) => s.get(Self::get_linear_index(x, y, z)),
            None => 0,
        }
    }

    pub fn to_proto_chunk_section(&self) -> data::ChunkSection {
        let mut blocks = [0; CHUNK_SECTION_BLOCK_COUNT * 2];

        let mut blocks_index = 0;
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    let b = self.get_block(x, y, z);
                    let short = b.to_u16();
                    // little endian
                    blocks[blocks_index] = short as u8;
                    blocks[blocks_index + 1] = (short >> 8) as u8;
                    blocks_index += 2;
                }
            }
        }

        let mut block_light = [0; CHUNK_SECTION_BLOCK_COUNT / 2];
        for (i, &light) in self.block_light.as_bytes().iter().enumerate() {
            block_light[i] = light;
        }

        let mut sky_light = None;

        if let Some(ref s) = self.sky_light {
            let mut temp = [0; CHUNK_SECTION_BLOCK_COUNT / 2];
            for (i, &light) in s.as_bytes().iter().enumerate() {
                temp[i] = light;
            }
            sky_light = Some(temp);
        }

        data::ChunkSection {
            blocks,
            block_light,
            sky_light,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> ChunkPos {
        ChunkPos { x, z }
    }
}

impl From<Vec3> for ChunkPos {
    fn from(pos: Vec3) -> Self {
        ChunkPos::new((pos.x / 16.0).floor() as i32, (pos.z / 16.0).floor() as i32)
    }
}

impl From<BlockPos> for ChunkPos {
    fn from(pos: BlockPos) -> Self {
        ChunkPos::new(
            (pos.x as f64 / 16.0).floor() as i32,
            (pos.z as f64 / 16.0).floor() as i32,
        )
    }
}

#[derive(Debug)]
pub struct Chunk {
    pos: ChunkPos,
    sections: [Option<Box<ChunkSection>>; 16],
    /// biomes as zx
    biomes: [[u8; 16]; 16],
    has_sky_light: bool,
    players_in_vicinity: HashSet<Uuid>,
    players: HashSet<Uuid>,
    block_entities: HashMap<(u8, u8, u8), Box<dyn BlockEntity>>,
}

impl Chunk {
    pub fn new(pos: ChunkPos, has_sky_light: bool) -> Chunk {
        Chunk {
            pos,
            sections: Default::default(),
            biomes: [[0; 16]; 16],
            has_sky_light,
            players_in_vicinity: HashSet::new(),
            players: HashSet::new(),
            block_entities: HashMap::new(),
        }
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }

    /// returns true when there is no player who see this chunk.
    pub fn is_abandoned(&self) -> bool {
        self.players_in_vicinity.len() == 0
    }

    fn get_section(
        sections: &[Option<Box<ChunkSection>>; 16],
        y: u8,
    ) -> &Option<Box<ChunkSection>> {
        &sections[(y / 16) as usize]
    }

    fn get_section_mut(
        sections: &mut [Option<Box<ChunkSection>>; 16],
        y: u8,
    ) -> &mut Option<Box<ChunkSection>> {
        &mut sections[(y / 16) as usize]
    }

    pub fn insert_player(&mut self, p: Uuid) {
        self.players.insert(p);
    }

    pub fn remove_player(&mut self, p: &Uuid) {
        self.players.remove(p);
    }

    pub fn players_iter(&self) -> impl Iterator<Item = &Uuid> {
        self.players.iter()
    }

    pub fn insert_player_in_vicinity(&mut self, p: Uuid) {
        self.players_in_vicinity.insert(p);
    }

    pub fn remove_player_in_vicinity(&mut self, p: &Uuid) {
        self.players_in_vicinity.remove(p);
    }

    pub fn players_in_vicinity_iter(&self) -> impl Iterator<Item = &Uuid> {
        self.players_in_vicinity.iter()
    }

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> BlockStateId {
        let sec = Self::get_section(&self.sections, y);
        match *sec {
            Some(ref sec) => sec.get_block(x, y % 16, z),
            None => BlockStateId::AIR,
        }
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, b: BlockStateId) {
        let has_sky_light = self.has_sky_light;

        let sec = Self::get_section_mut(&mut self.sections, y);
        match *sec {
            Some(ref mut s) => {
                s.set_block(x, y % 16, z, b);
                let block_entity = b.get_type().create_new_block_entity();
                if let Some(block_entity) = block_entity {
                    self.block_entities.insert((x, y, z), block_entity);
                } else {
                    self.block_entities.remove(&(x, y, z));
                }
            }
            None => {
                if b.get_type() == Block::Air {
                    return;
                }

                let mut section = ChunkSection::new(has_sky_light);
                section.set_block(x, y % 16, z, b);
                let block_entity = b.get_type().create_new_block_entity();
                if let Some(block_entity) = block_entity {
                    self.block_entities.insert((x, y, z), block_entity);
                } else {
                    self.block_entities.remove(&(x, y, z));
                }
                *sec = Some(Box::new(section));
                return;
            }
        }
    }

    pub fn get_block_light(&self, x: u8, y: u8, z: u8) -> u8 {
        let sec = Self::get_section(&self.sections, y);
        match *sec {
            Some(ref sec) => sec.get_block_light(x, y % 16, z),
            None => 0,
        }
    }

    pub fn set_block_light(&mut self, x: u8, y: u8, z: u8, light: u8) {
        let sec = Self::get_section_mut(&mut self.sections, y);
        if let Some(ref mut sec) = *sec {
            sec.set_block_light(x, y % 16, z, light);
        }
    }

    pub fn get_sky_light(&self, x: u8, y: u8, z: u8) -> u8 {
        let sec = Self::get_section(&self.sections, y);
        match *sec {
            Some(ref sec) => sec.get_sky_light(x, y % 16, z),
            None => 0,
        }
    }

    pub fn set_sky_light(&mut self, x: u8, y: u8, z: u8, light: u8) {
        let sec = Self::get_section_mut(&mut self.sections, y);
        if let Some(ref mut sec) = *sec {
            sec.set_sky_light(x, y % 16, z, light);
        }
    }

    pub const FULL_BIT_MASK: u16 = !0;

    /// returns a ChunkData packet.
    /// bit_mask hints at what chunk sections to send. If it is a FULL_BIT_MASK, ground_up_continuous is sent where empty chunk sections aren't sent.
    /// If not, chunk sections are sent even if they are empty, but not if it was never created. (null)
    pub fn to_proto_chunk_data(&self, bit_mask: u16) -> SPacket {
        let ground_up_continuous = bit_mask == Self::FULL_BIT_MASK;

        let mut primary_bit_mask = 0;
        let data = {
            let mut sections =
                Vec::with_capacity(self.sections.iter().filter(|x| x.is_some()).count());
            for (i, sec) in self.sections.iter().enumerate() {
                if let Some(ref s) = *sec {
                    if bit_mask & (1 << i) != 0 && !(ground_up_continuous && s.is_empty()) {
                        primary_bit_mask |= 1 << i;
                        sections.push(s.to_proto_chunk_section());
                    }
                }
            }

            if ground_up_continuous {
                SPlayChunkDataData::GroundUpContinuous(GroundUpContinuous {
                    sections: GroundUpNonContinuous { sections },
                    biome_array: Box::new(self.biomes),
                })
            } else {
                SPlayChunkDataData::GroundUpNonContinuous(GroundUpNonContinuous { sections })
            }
        };

        SPacket::PlayChunkData {
            chunk_x: self.pos.x,
            chunk_z: self.pos.z,
            primary_bit_mask,
            data,
        }
    }

    pub fn to_proto_map_chunk_bulk_data(&self) -> SPlayMapChunkBulkData {
        let mut primary_bit_mask = 0;
        let chunk = {
            let mut sections = Vec::with_capacity(
                self.sections
                    .iter()
                    .filter(|x| x.is_some() && !x.as_ref().unwrap().is_empty())
                    .count(),
            );
            for (i, sec) in self.sections.iter().enumerate() {
                if let Some(ref s) = *sec {
                    if !s.is_empty() {
                        primary_bit_mask |= 1 << i;
                        sections.push(s.to_proto_chunk_section());
                    }
                }
            }

            GroundUpContinuous {
                sections: GroundUpNonContinuous { sections },
                biome_array: Box::new(self.biomes),
            }
        };

        SPlayMapChunkBulkData {
            chunk_x: self.pos.x,
            chunk_z: self.pos.z,
            primary_bit_mask,
            chunk,
        }
    }

    pub fn empty_proto_chunk_data(pos: ChunkPos) -> SPacket {
        SPacket::PlayChunkData {
            chunk_x: pos.x,
            chunk_z: pos.z,
            primary_bit_mask: 0,
            data: SPlayChunkDataData::GroundUpContinuous(GroundUpContinuous {
                sections: GroundUpNonContinuous {
                    sections: Vec::new(),
                },
                biome_array: Box::new([[0; 16]; 16]),
            }),
        }
    }
}
