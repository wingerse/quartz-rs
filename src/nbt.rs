use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::error::Error;
use std::fmt;
use std::string::FromUtf8Error;

use binary;

fn write_string<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    binary::write_ushort(w, s.len() as u16)?;
    w.write_all(s.as_bytes())
}

fn read_string<R: Read>(r: &mut R) -> Result<String, NbtError> {
    let len = binary::read_ushort(r)?;
    let mut buf = Vec::with_capacity(len as usize);
    r.read_exact(&mut buf)?;
    let string = String::from_utf8(buf)?;
    Ok(string)
}

const MAX_NEXT_LEVEL: u32 = 512;

#[derive(Debug)]
pub enum NbtError {
    ByteArrayNegativeLength(i32),
    InvalidUtfEncodedString,
    MaxNestLevelReached(u32),
    IntArrayNegativeLength(i32),
    InvalidID(u8),
    NotACompoundID(u8),
    IOErr(io::Error),
}

impl fmt::Display for NbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NbtError::ByteArrayNegativeLength(x) => {
                write!(f, "Byte array with negative length: {}", x)
            }
            NbtError::InvalidUtfEncodedString => write!(f, "Invalid utf encoded string"),
            NbtError::MaxNestLevelReached(x) => write!(f, "Max nest level allowed reached: {}", x),
            NbtError::IntArrayNegativeLength(x) => {
                write!(f, "Int array with negative length: {}", x)
            }
            NbtError::InvalidID(x) => write!(f, "Tag with invalid ID: {}", x),
            NbtError::NotACompoundID(x) => write!(f, "Root tag is not a compound. Got id: {}", x),
            NbtError::IOErr(ref e) => e.fmt(f),
        }
    }
}

impl Error for NbtError {
    fn description(&self) -> &str {
        match *self {
            NbtError::IOErr(ref e) => e.description(),
            _ => "Nbt Error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            NbtError::IOErr(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for NbtError {
    fn from(err: io::Error) -> Self {
        NbtError::IOErr(err)
    }
}

impl From<FromUtf8Error> for NbtError {
    fn from(_: FromUtf8Error) -> Self {
        NbtError::InvalidUtfEncodedString
    }
}

#[derive(Debug)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
}

impl Tag {
    fn id(&self) -> u8 {
        match *self {
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
        }
    }

    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            Tag::Byte(x) => binary::write_byte(w, x)?,
            Tag::Short(x) => binary::write_ishort(w, x)?,
            Tag::Int(x) => binary::write_int(w, x)?,
            Tag::Long(x) => binary::write_long(w, x)?,
            Tag::Float(x) => binary::write_float(w, x)?,
            Tag::Double(x) => binary::write_double(w, x)?,
            Tag::ByteArray(ref x) => {
                binary::write_int(w, x.len() as i32)?;
                w.write_all(&x)?;
            }
            Tag::String(ref x) => write_string(w, x)?,
            Tag::List(ref x) => {
                let (item_id, len) = if x.len() == 0 {
                    (0, 0)
                } else {
                    (x[0].id(), x.len())
                };

                binary::write_ubyte(w, item_id)?;
                binary::write_int(w, len as i32)?;

                for tag in x.iter() {
                    tag.write(w)?;
                }
            }
            Tag::Compound(ref x) => {
                for (k, v) in x.iter() {
                    binary::write_ubyte(w, v.id())?;
                    write_string(w, k)?;
                    v.write(w)?;
                }
            },
            Tag::IntArray(ref x) => {
                binary::write_int(w, x.len() as i32)?;
                for v in x.iter() {
                    binary::write_int(w, *v)?;
                }
            }
        };

        Ok(())
    }

    fn read<R: Read>(id: u8, r: &mut R, mut level: u32) -> Result<Tag, NbtError> {
        match id {
            1 => Ok(Tag::Byte(binary::read_byte(r)?)),
            2 => Ok(Tag::Short(binary::read_ishort(r)?)),
            3 => Ok(Tag::Int(binary::read_int(r)?)),
            4 => Ok(Tag::Long(binary::read_long(r)?)),
            5 => Ok(Tag::Float(binary::read_float(r)?)),
            6 => Ok(Tag::Double(binary::read_double(r)?)),
            7 => {
                let len = binary::read_int(r)?;
                if len < 0 {
                    return Err(NbtError::ByteArrayNegativeLength(len));
                }
                let mut v = Vec::<u8>::with_capacity(len as usize);
                r.read_exact(&mut v)?;
                Ok(Tag::ByteArray(v))
            }
            8 => Ok(Tag::String(read_string(r)?)),
            9 => {
                level += 1;
                if level > MAX_NEXT_LEVEL {
                    return Err(NbtError::MaxNestLevelReached(level));
                }

                let id = binary::read_ubyte(r)?;
                let mut len = binary::read_int(r)?;
                if len < 0 {
                    len = 0;
                }

                let mut v = Vec::with_capacity(len as usize);
                for t in v.iter_mut() {
                    *t = Tag::read(id, r, level)?;
                }
                Ok(Tag::List(v))
            }
            10 => {
                level += 1;
                if level > MAX_NEXT_LEVEL {
                    return Err(NbtError::MaxNestLevelReached(level));
                }

                let mut compound = HashMap::new();

                loop {
                    let id = binary::read_ubyte(r)?;
                    if id == 0 {
                        break;
                    }

                    let name = read_string(r)?;
                    let tag = Tag::read(id, r, level)?;
                    compound.insert(name, tag);
                }
                Ok(Tag::Compound(compound))
            }, 
            11 => {
                let len = binary::read_int(r)?;
                if len < 0 {
                    return Err(NbtError::IntArrayNegativeLength(len));
                }
                let mut v = Vec::with_capacity(len as usize);
                
                for i in v.iter_mut() {
                    *i = binary::read_int(r)?;
                }
                Ok(Tag::IntArray(v))
            },
            _ => Err(NbtError::InvalidID(id)),
        }
    }
}

pub struct NBT(Option<(String, Tag)>);

impl NBT {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self.0 {
            None => binary::write_ubyte(w, 0),
            Some((ref name, ref tag)) => {
                binary::write_ubyte(w, tag.id())?;
                write_string(w, name)?;
                tag.write(w)
            }
        }
    }

    pub fn read<R: Read>(r: &mut R) -> Result<NBT, NbtError> {
        let id = binary::read_ubyte(r)?;
        if id == 0 {
            return Ok(NBT(None));
        }
        if id != 10 {
            return Err(NbtError::NotACompoundID(id));
        }

        let name = read_string(r)?;
        let root = Tag::read(id, r, 0)?;

        Ok(NBT(Some((name, root))))
    }
}