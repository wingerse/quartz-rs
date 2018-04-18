use binary;
use nbt;
use proto::Result;
use std::fmt::{self, Debug};
use std::io::{self, BufRead, Read, Write};
use uuid;

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
    binary::write_byte(w, ((f64::from(a.round() as i32 % 360) / 360.0) * 256.0) as i8)
}

pub fn read_angle<R: Read>(r: &mut R) -> io::Result<f64> {
    let b = binary::read_byte(r)?;
    Ok(f64::from(b) * (360.0 / 256.0))
}

#[derive(Debug)]
pub enum SlotData {
    Empty,
    Some {
        block_id: i16,
        item_count: i8,
        item_damage: i16,
        nbt: nbt::NBT,
    }
}

impl SlotData {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            SlotData::Empty => {
                binary::write_ishort(w, -1)?;
            },
            SlotData::Some {
                block_id,
                item_count,
                item_damage,
                ref nbt
            } => {
                binary::write_ishort(w, block_id)?;
                binary::write_byte(w, item_count)?;
                binary::write_ishort(w, item_damage)?;
                nbt.write(w)?;
            },
        }

        Ok(())
    }

    pub fn read<R: BufRead>(r: &mut R) -> Result<SlotData> {
        let block_id = binary::read_ishort(r)?;
        if block_id == -1 {
            return Ok(SlotData::Empty);
        }

        let item_count = binary::read_byte(r)?;
        let item_damage = binary::read_ishort(r)?;
        let nbt = nbt::NBT::read(r)?;

        Ok(SlotData::Some{block_id, item_count, item_damage, nbt})
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let mut val: u64 = (self.z & 0x3ff_ffff) as u64;
        val |= (self.y as u64 & 0xfff) << 26;
        val |= (self.x as u64 & 0x3ff_ffff) << 38;
        binary::write_long(w, val as i64)
    }

    pub fn read<R: Read>(r: &mut R) -> Result<Position> {
        let val = binary::read_long(r)?;
        let x = (val >> 38) as i32;
        let y = (val << 26 >> 38) as i32;
        let z = (val as i32) << 6 >> 6;
        Ok(Position {x, y, z})
    }
}

#[derive(Debug)]
pub enum MetadataEntry {
    Byte(i8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    Slot(SlotData),
    Pos{
        x: i32,
        y: i32,
        z: i32,
    },
    Orientation {
        pitch: f32,
        yaw: f32,
        roll: f32,
    }
}

impl MetadataEntry {
    fn get_type(&self) -> u8 {
        match *self {
            MetadataEntry::Byte(_) => 0,
            MetadataEntry::Short(_) => 1,
            MetadataEntry::Int(_) => 2,
            MetadataEntry::Float(_) => 3,
            MetadataEntry::String(_) => 4,
            MetadataEntry::Slot(_) => 5,
            MetadataEntry::Pos{..} => 6,
            MetadataEntry::Orientation {..} => 7
        }
    }
}

#[derive(Debug)]
pub struct EntityMetadata(Vec<MetadataEntry>);

impl EntityMetadata {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for (i, v) in self.0.iter().enumerate() {
            let byte = (i as u8 & 0x1f) | (v.get_type() << 5);
            binary::write_ubyte(w, byte)?;
            match *v {
                MetadataEntry::Byte(x) => binary::write_byte(w, x)?,
                MetadataEntry::Short(x) => binary::write_ishort(w, x)?,
                MetadataEntry::Int(x) => binary::write_int(w, x)?,
                MetadataEntry::Float(x) => binary::write_float(w, x)?,
                MetadataEntry::String(ref x) => write_string(w, &x)?,
                MetadataEntry::Slot(ref x) => x.write(w)?,
                MetadataEntry::Pos{x, y, z} => {
                    binary::write_int(w, x)?;
                    binary::write_int(w, y)?;
                    binary::write_int(w, z)?;
                },
                MetadataEntry::Orientation {pitch, yaw, roll} => {
                    binary::write_float(w, pitch)?;
                    binary::write_float(w, yaw)?;
                    binary::write_float(w, roll)?;
                },
            }
        }
        binary::write_ubyte(w, 127)?;
        Ok(())
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

pub const CHUNK_SECTION_BLOCK_COUNT: usize = 16 * 16 * 16;

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