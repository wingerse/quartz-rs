use std::io::{self, Write};
use std::collections::HashMap;

use crate::proto::data::SlotData;
use crate::binary;
use crate::proto::data;

#[derive(Debug)]
pub enum MetadataEntry {
    Byte(i8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    Slot(SlotData),
    Pos {
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
pub struct EntityMetadata(HashMap<u8, MetadataEntry>);

impl EntityMetadata {
    pub fn new() -> EntityMetadata {
        EntityMetadata(HashMap::new())
    }

    pub fn insert(&mut self, key: u8, value: MetadataEntry) {
        self.0.insert(key, value);
    }

    pub fn write_proto<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for (&i, v) in self.0.iter() {
            let byte = (i as u8 & 0x1f) | (v.get_type() << 5);
            binary::write_ubyte(w, byte)?;
            match *v {
                MetadataEntry::Byte(x) => binary::write_byte(w, x)?,
                MetadataEntry::Short(x) => binary::write_ishort(w, x)?,
                MetadataEntry::Int(x) => binary::write_int(w, x)?,
                MetadataEntry::Float(x) => binary::write_float(w, x)?,
                MetadataEntry::String(ref x) => data::write_string(w, &x)?,
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