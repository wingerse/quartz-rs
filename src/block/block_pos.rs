use std::io::{self, Write, Read};

use binary;
use proto;

use super::Facing;

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

    pub fn offset(&self, facing: Facing) -> Option<BlockPos> {
        match facing {
            Facing::Down => if self.y == 0 { None } else { Some(BlockPos::new(self.x, self.y - 1, self.z)) },
            Facing::Up => if self.y == 255 { None } else { Some(BlockPos::new(self.x, self.y + 1, self.z)) },
            Facing::North => Some(BlockPos::new(self.x, self.y, self.z - 1)),
            Facing::South => Some(BlockPos::new(self.x, self.y, self.z + 1)),
            Facing::West => Some(BlockPos::new(self.x - 1, self.y, self.z)),
            Facing::East => Some(BlockPos::new(self.x + 1, self.y, self.z)),
        }
    }
}