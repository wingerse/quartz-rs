use binary;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::io::{self, Read, Write};
use std::string::FromUtf8Error;

fn write_string<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    binary::write_ushort(w, s.len() as u16)?;
    w.write_all(s.as_bytes())
}

fn read_string<R: Read>(r: &mut R) -> Result<String, Error> {
    let len = binary::read_ushort(r)?;
    let mut buf = vec![0u8; len as usize];
    r.read_exact(&mut buf)?;
    let string = String::from_utf8(buf)?;
    Ok(string)
}

const MAX_NEXT_LEVEL: u32 = 512;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ByteArrayNegativeLength(err: i32) {
            description("byte array with negative length")
            display(me) -> ("{}: {}", me.description(), err)
        }
        InvalidUtf8String(err: FromUtf8Error) {
            description(err.description())
            display("invalid utf8 string: {}", err)
            from()
            cause(err)
        }
        MaxNestLevelReached(err: u32) {
            description("max nest level allowed reached")
            display(me) -> ("{}: {}", me.description(), err)
        }
        IntArrayNegativeLength(err: i32) {
            description("int array with negative length")
            display(me) -> ("{}: {}", me.description(), err)
        }
        InvalidID(err: u8) {
            description("tag with invalid ID")
            display(me) -> ("{}: {}", me.description(), err)
        }
        NotACompoundID(err: u8) {
            description("root tag is not a compound")
            display(me) -> ("{}, got: {}", me.description(), err)
        }
        IOErr(err: io::Error) {
            description(err.description())
            display("io error: {}", err)
            from()
            cause(err)
        }
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
                let (item_id, len) = if x.is_empty() {
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
                binary::write_ubyte(w, 0)?;
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

    fn read<R: Read>(id: u8, r: &mut R, mut level: u32) -> Result<Tag, Error> {
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
                    return Err(Error::ByteArrayNegativeLength(len));
                }
                let mut v = vec![0u8; len as usize];
                r.read_exact(&mut v)?;
                Ok(Tag::ByteArray(v))
            }
            8 => Ok(Tag::String(read_string(r)?)),
            9 => {
                level += 1;
                if level > MAX_NEXT_LEVEL {
                    return Err(Error::MaxNestLevelReached(level));
                }

                let id = binary::read_ubyte(r)?;
                let mut len = binary::read_int(r)?;
                if len < 0 {
                    len = 0;
                }

                let mut v = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    v.push(Tag::read(id, r, level)?);
                }
                Ok(Tag::List(v))
            }
            10 => {
                level += 1;
                if level > MAX_NEXT_LEVEL {
                    return Err(Error::MaxNestLevelReached(level));
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
                    return Err(Error::IntArrayNegativeLength(len));
                }
                let mut v = vec![0i32; len as usize];

                for i in &mut v {
                    *i = binary::read_int(r)?;
                }
                Ok(Tag::IntArray(v))
            },
            _ => Err(Error::InvalidID(id)),
        }
    }
}

/// A named binary tag. This can be empty(None) or have a single tag
#[derive(Debug)]
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

    pub fn read<R: Read>(r: &mut R) -> Result<NBT, Error> {
        let id = binary::read_ubyte(r)?;
        if id == 0 {
            return Ok(NBT(None));
        }
        if id != 10 {
            return Err(Error::NotACompoundID(id));
        }

        let name = read_string(r)?;
        let root = Tag::read(id, r, 0)?;

        Ok(NBT(Some((name, root))))
    }
}