use collections::{NibbleArray, VarbitArray};
use proto::data::{self, GroundUpContinuous, GroundUpNonContinuous};
use proto::packets::{SPacket, SPlayChunkDataData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockID {
    typ: u8,
    meta: u8,
}

impl BlockID {
    pub const AIR: BlockID = BlockID { typ: 0, meta: 0 };
    
    pub fn new(typ: u8, meta: u8) -> BlockID {
        BlockID {
            typ,
            meta: meta & 0x0f,
        }
    }

    pub fn get_type(&self) -> u8 { self.typ }

    pub fn set_type(&mut self, typ: u8) { self.typ = typ; }

    pub fn get_meta(&self) -> u8 { self.meta }

    pub fn set_meta(&mut self, meta: u8) { self.meta = meta & 0x0f; }

    pub fn as_u16(&self) -> u16 { ((self.typ as u16) << 4) | self.meta as u16 }
}

pub const CHUNK_SECTION_BLOCK_COUNT: usize = 16 * 16 * 16;

#[derive(Debug)]
pub struct ChunkSection {
    // this is not used in 1.8 but saves memory as much as 2x, sacrificing performance a bit.
    palette: Vec<BlockID>,
    blocks: VarbitArray,
    block_light: NibbleArray,
    sky_light: Option<NibbleArray>,
    air_count: u16,
}

impl ChunkSection {
    pub fn new(has_sky_light: bool) -> ChunkSection {
        ChunkSection {
            palette: vec![BlockID::AIR],
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

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> BlockID {
        self.palette[self.blocks.get(Self::get_linear_index(x, y, z)) as usize]
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, b: BlockID) {
        let previous = self.get_block(x, y, z);
        if previous == b {
            return;
        }

        if previous == BlockID::AIR {
            self.air_count -= 1;
        } else if b == BlockID::AIR {
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

            self.blocks.set(Self::get_linear_index(x, y, z), (self.palette.len() - 1) as u64);
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
                    let short = b.as_u16();
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

        data::ChunkSection { blocks, block_light, sky_light }
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

#[derive(Debug)]
pub struct Chunk {
    pos: ChunkPos,
    sections: [Option<Box<ChunkSection>>; 16],
    /// biomes as zx
    biomes: [[u8; 16]; 16],
    has_sky_light: bool,
}

impl Chunk {
    pub fn new(pos: ChunkPos, has_sky_light: bool) -> Chunk {
        Chunk { pos, sections: Default::default(), biomes: [[0; 16]; 16], has_sky_light }
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }

    fn get_section(&self, y: u8) -> &Option<Box<ChunkSection>> {
        &self.sections[(y / 16) as usize]
    }

    fn get_section_mut(&mut self, y: u8) -> &mut Option<Box<ChunkSection>> {
        &mut self.sections[(y / 16) as usize]
    }

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> BlockID {
        let sec = self.get_section(y);
        match *sec {
            Some(ref sec) => sec.get_block(x, y % 16, z),
            None => BlockID::AIR
        }
    }

    pub fn set_block(&mut self, x: u8, y: u8, z: u8, b: BlockID) {
        let has_sky_light = self.has_sky_light;

        let sec = self.get_section_mut(y);
        match *sec {
            Some(ref mut s) => {
                s.set_block(x, y % 16, z, b);
            }
            None => {
                if b == BlockID::AIR {
                    return;
                }

                let mut section = ChunkSection::new(has_sky_light);
                section.set_block(x, y % 16, z, b);
                *sec = Some(Box::new(section));
                return;
            }
        }
    }

    pub fn get_block_light(&self, x: u8, y: u8, z: u8) -> u8 {
        let sec = self.get_section(y);
        match *sec {
            Some(ref sec) => sec.get_block_light(x, y % 16, z),
            None => 0,
        }
    }

    pub fn set_block_light(&mut self, x: u8, y: u8, z: u8, light: u8) {
        let sec = self.get_section_mut(y);
        if let Some(ref mut sec) = *sec {
            sec.set_block_light(x, y % 16, z, light);
        }
    }

    pub fn get_sky_light(&self, x: u8, y: u8, z: u8) -> u8 {
        let sec = self.get_section(y);
        match *sec {
            Some(ref sec) => sec.get_sky_light(x, y % 16, z),
            None => 0,
        }
    }

    pub fn set_sky_light(&mut self, x: u8, y: u8, z: u8, light: u8) {
        let sec = self.get_section_mut(y);
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
            let mut sections = Vec::with_capacity(self.sections.iter().filter(|x| x.is_some()).count());
            for (i, sec) in self.sections.iter().enumerate() {
                if let Some(ref s) = *sec {
                    if bit_mask & (1 << i) != 0 && !(ground_up_continuous && s.is_empty()) {
                        primary_bit_mask |= (1 << i);
                        sections.push(s.to_proto_chunk_section());
                    }
                }
            }

            if ground_up_continuous {
                SPlayChunkDataData::GroundUpContinuous(GroundUpContinuous { sections: GroundUpNonContinuous { sections }, biome_array: Box::new(self.biomes) })
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

    pub fn empty_proto_chunk_data(&self) -> SPacket {
        SPacket::PlayChunkData {
            chunk_x: self.pos.x,
            chunk_z: self.pos.z,
            primary_bit_mask: 0,
            data: SPlayChunkDataData::GroundUpContinuous(GroundUpContinuous {
                sections: GroundUpNonContinuous { sections: Vec::new() },
                biome_array: Box::new(self.biomes)
            }),
        }
    }
}