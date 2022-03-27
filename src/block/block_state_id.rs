use crate::block::{Block};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlockStateId {
    typ: Block,
    meta: u8,
}

impl BlockStateId {
    pub const AIR: BlockStateId = BlockStateId { typ: Block::Air, meta: 0 };

    pub fn new(typ: Block, meta: u8) -> BlockStateId {
        BlockStateId {
            typ,
            meta: meta & 0x0f,
        }
    }

    pub fn get_type(&self) -> Block { self.typ }

    pub fn set_type(&mut self, typ: Block) { self.typ = typ; }

    pub fn get_meta(&self) -> u8 { self.meta }

    pub fn set_meta(&mut self, meta: u8) { self.meta = meta & 0x0f; }

    pub fn to_u16(&self) -> u16 { ((self.typ.to_u8() as u16) << 4) | self.meta as u16 }
}