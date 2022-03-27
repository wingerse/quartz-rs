use std::fmt::{self, Debug};
use std::io::{self, BufRead, Read, Write};

use uuid;

use crate::binary;
use crate::nbt::Nbt;
use crate::proto::Result;
use crate::world::chunk::CHUNK_SECTION_BLOCK_COUNT;

pub fn write_string<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    binary::write_varint(w, s.len() as i32)?;
    w.write_all(s.as_bytes())
}

pub fn read_string<R: BufRead>(r: &mut R) -> Result<String> {
    let len = binary::read_varint(r)?;
    let mut buf = vec![0u8; len as usize];
    r.read_exact(&mut buf)?;
    let string = String::from_utf8(buf)?;
    Ok(string)
}

pub fn write_uuid<W: Write>(w: &mut W, u: &uuid::Uuid) -> io::Result<()> {
    let mut arr = *u.as_bytes();
    arr.reverse();
    w.write_all(&arr[..])
}

pub fn read_uuid<R: Read>(r: &mut R) -> io::Result<uuid::Uuid> {
    let mut arr = [0; 16];
    r.read_exact(&mut arr[..])?;
    arr.reverse();
    // cannot return error when b is 16 bytes.
    Ok(uuid::Uuid::from_bytes(&arr[..]).unwrap())
}

pub fn write_angle<W: Write>(w: &mut W, a: f64) -> io::Result<()> {
    binary::write_byte(w, ((f64::from(a.round() as i32 % 360) / 360.0) * 256.0).floor() as i8)
}

pub fn read_angle<R: Read>(r: &mut R) -> io::Result<f64> {
    let b = binary::read_byte(r)?;
    Ok(f64::from(b) * (360.0 / 256.0))
}

#[derive(Debug)]
pub enum SlotData {
    Empty,
    Some {
        id: i16,
        item_count: i8,
        item_damage: i16,
        tag: Nbt,
    }
}

impl SlotData {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            SlotData::Empty => {
                binary::write_ishort(w, -1)?;
            },
            SlotData::Some {
                id,
                item_count,
                item_damage,
                ref tag
            } => {
                binary::write_ishort(w, id)?;
                binary::write_byte(w, item_count)?;
                binary::write_ishort(w, item_damage)?;
                tag.write(w)?;
            },
        }

        Ok(())
    }

    pub fn read<R: BufRead>(r: &mut R) -> Result<SlotData> {
        let id = binary::read_ishort(r)?;
        if id == -1 {
            return Ok(SlotData::Empty);
        }

        let item_count = binary::read_byte(r)?;
        let item_damage = binary::read_ishort(r)?;
        let tag = Nbt::read(r)?;

        Ok(SlotData::Some{ id, item_count, item_damage, tag })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModifierData {
    pub uuid: uuid::Uuid,
    pub amount: f64,
    pub operation: i8,
}

impl ModifierData {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write_uuid(w, &self.uuid)?;
        binary::write_double(w, self.amount)?;
        binary::write_byte(w, self.operation)
    }

    pub fn read<R: Read>(r: &mut R) -> io::Result<ModifierData> {
        let uuid = read_uuid(r)?;
        let amount = binary::read_double(r)?;
        let operation = binary::read_byte(r)?;
        Ok(ModifierData{uuid, amount, operation})
    }
}

pub struct ChunkSection {
    pub blocks: [u8; CHUNK_SECTION_BLOCK_COUNT * 2],
    pub block_light: [u8; CHUNK_SECTION_BLOCK_COUNT / 2],
    pub sky_light: Option<[u8; CHUNK_SECTION_BLOCK_COUNT / 2]>,
}

impl ChunkSection {
    pub fn len(&self) -> usize {
        let mut l = 0;
        l += self.blocks.len();
        l += self.block_light.len();
        l += match self.sky_light {
            Some(ref arr) => arr.len(),
            None => 0,
        };

        l
    }
}

impl Debug for ChunkSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChunkSection")
            .field("blocks", &&self.blocks[..])
            .field("block_light", &&self.block_light[..])
            .field("sky_light", &self.sky_light.as_ref().map(|arr| &arr[..]))
            .finish()
    }
}

#[derive(Debug)]
pub struct GroundUpContinuous {
    pub sections: GroundUpNonContinuous,
    pub biome_array: Box<[[u8; 16]; 16]>,
}

impl GroundUpContinuous {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.sections.write(w)?;
        for z in &*self.biome_array {
            w.write_all(z)?;
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.sections.len() + (16 * 16)
    }
}

#[derive(Debug)]
pub struct GroundUpNonContinuous {
    pub sections: Vec<ChunkSection>,
}

impl GroundUpNonContinuous {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for sec in &self.sections {
            w.write_all(&sec.blocks)?;
        }
        for sec in &self.sections {
            w.write_all(&sec.block_light)?;
        }
        for sec in &self.sections {
            if let Some(ref sky_light) = sec.sky_light {
                w.write_all(sky_light)?;
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.sections.iter().fold(0, |acc, x| acc + x.len())
    }
}