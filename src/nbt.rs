use std::collections::HashMap;
use std::error::Error as StdError;
use std::io::{self, Read, Write};
use std::string::FromUtf8Error;

use crate::binary;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "byte array with negative length: {}", _0)]
    ByteArrayNegativeLength(i32),
    #[fail(display = "invalid utf8 string: {}", _0)]
    InvalidUtf8String(#[cause] FromUtf8Error),
    #[fail(display = "max nest level allowed reached: {}", _0)]
    MaxNestLevelReached(u32),
    #[fail(display = "int array with negative length: {}", _0)]
    IntArrayNegativeLength(i32),
    #[fail(display = "root tag is not a compound, got: {}", _0)]
    InvalidRootTag(u8),
    #[fail(display = "tag with invalid ID: {}", _0)]
    InvalidID(u8),
    #[fail(display = "io error: {}", _0)]
    IOErr(#[cause] io::Error),
}

impl_from_for_newtype_enum!(Error::InvalidUtf8String, FromUtf8Error);
impl_from_for_newtype_enum!(Error::IOErr, io::Error);

#[derive(Debug, Fail)]
pub enum DeserializeError {
    #[fail(display = "field \"{}\" not found", _0)]
    FieldNotFound(String),
    #[fail(display = "type mismatched. expected {}, got {}", _0, _1)]
    TypeMismatch(&'static str, &'static str),
    #[fail(display = "index out of bounds: the len is {} but the index is {}", _0, _0)]
    OutOfBounds(usize, usize),
}

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

#[derive(Debug)]
pub struct Compound(pub HashMap<String, Tag>);

impl Compound {
    pub fn get(&self, field: &str) -> Result<&Tag, DeserializeError> {
        self.0.get(field).ok_or(DeserializeError::FieldNotFound(field.into()))
    }

    pub fn contains_key(&self, field: &str) -> bool {
        self.0.contains_key(field)
    }

    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for (k, v) in self.0.iter() {
            binary::write_ubyte(w, v.id())?;
            write_string(w, k)?;
            v.write(w)?;
        }
        binary::write_ubyte(w, 0)
    }

    fn read<R: Read>(r: &mut R, mut level: u32) -> Result<Compound, Error> {
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
        Ok(Compound(compound))
    }
}

/// This is the root tag. Similar to compound except it has only 1 tag which is a compound and no end byte.
#[derive(Debug)]
pub enum Nbt {
    Empty,
    Some(String, Compound),
}

impl Nbt {
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            Nbt::Empty => binary::write_ubyte(w, 0),
            Nbt::Some(ref name, ref root) => {
                binary::write_ubyte(w, 10)?;
                write_string(w, &name)?;
                root.write(w)
            }
        }
    }

    pub fn read<R: Read>(r: &mut R) -> Result<Nbt, Error> {
        let id = binary::read_ubyte(r)?;
        if id == 0 {
            return Ok(Nbt::Empty);
        }
        if id != 10 {
            return Err(Error::InvalidRootTag(id));
        }
        let name = read_string(r)?;
        let root = Compound::read(r, 0)?;
        Ok(Nbt::Some(name, root))
    }
}

#[derive(Debug)]
pub struct List(pub Vec<Tag>);

impl List {
    pub fn get(&self, index: usize) -> Result<&Tag, DeserializeError> {
        let len = self.0.len();
        self.0.get(index).ok_or(DeserializeError::OutOfBounds(len, index))
    }

    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let (item_id, len) = if self.0.is_empty() {
            (0, 0)
        } else {
            (self.0[0].id(), self.0.len())
        };

        binary::write_ubyte(w, item_id)?;
        binary::write_int(w, len as i32)?;

        for tag in self.0.iter() {
            tag.write(w)?;
        }

        Ok(())
    }

    fn read<R: Read>(r: &mut R, mut level: u32) -> Result<List, Error> {
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
        Ok(List(v))
    }
}

#[derive(Debug)]
pub struct ByteArray(pub Vec<u8>);

impl ByteArray {
    pub fn get(&self, index: usize) -> Result<u8, DeserializeError> {
        let len = self.0.len();
        self.0.get(index).map(|x| *x).ok_or(DeserializeError::OutOfBounds(len, index))
    }

    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        binary::write_int(w, self.0.len() as i32)?;
        w.write_all(&self.0)
    }

    fn read<R: Read>(r: &mut R) -> Result<Self, Error> {
        let len = binary::read_int(r)?;
        if len < 0 {
            return Err(Error::ByteArrayNegativeLength(len));
        }
        let mut v = vec![0u8; len as usize];
        r.read_exact(&mut v)?;
        Ok(ByteArray(v))
    }
}

#[derive(Debug)]
pub struct IntArray(pub Vec<i32>);

impl IntArray {
    pub fn get(&self, index: usize) -> Result<i32, DeserializeError> {
        let len = self.0.len();
        self.0.get(index).map(|x| *x).ok_or(DeserializeError::OutOfBounds(len, index))
    }

    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        binary::write_int(w, self.0.len() as i32)?;
        for v in self.0.iter() {
            binary::write_int(w, *v)?;
        }
        Ok(())
    }

    fn read<R: Read>(r: &mut R) -> Result<Self, Error> {
        let len = binary::read_int(r)?;
        if len < 0 {
            return Err(Error::IntArrayNegativeLength(len));
        }
        let mut v = vec![0i32; len as usize];

        for i in &mut v {
            *i = binary::read_int(r)?;
        }
        Ok(IntArray(v))
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
    ByteArray(ByteArray),
    String(String),
    List(List),
    Compound(Compound),
    IntArray(IntArray),
}

impl From<bool> for Tag {
    fn from(x: bool) -> Tag { Tag::Byte(if x { 1 } else { 0 }) }
}

impl_from_for_newtype_enum!(Tag::Byte, i8);
impl_from_for_newtype_enum!(Tag::Short, i16);
impl_from_for_newtype_enum!(Tag::Int, i32);
impl_from_for_newtype_enum!(Tag::Long, i64);
impl_from_for_newtype_enum!(Tag::Float, f32);
impl_from_for_newtype_enum!(Tag::Double, f64);
impl_from_for_newtype_enum!(Tag::ByteArray, ByteArray);
impl_from_for_newtype_enum!(Tag::String, String);
impl_from_for_newtype_enum!(Tag::List, List);
impl_from_for_newtype_enum!(Tag::Compound, Compound);
impl_from_for_newtype_enum!(Tag::IntArray, IntArray);

macro_rules! impl_as {
    ($(($func:ident;$tag:ident;$res:ty))*) => {
        $(
        pub fn $func(&self) -> Result<$res, DeserializeError> {
            if let Tag::$tag(x) = *self { Ok(x) } else { Err(DeserializeError::TypeMismatch(stringify!($tag), self.name())) }
        })*
    };
}

macro_rules! impl_as_ref {
    ($(($func:ident;$tag:ident;$res:ty))*) => {
        $(pub fn $func(&self) -> Result<&$res, DeserializeError> {
            if let Tag::$tag(ref x) = *self { Ok(x) } else { Err(DeserializeError::TypeMismatch(stringify!($tag), self.name())) }
        })*
    };
}

impl Tag {
    fn name(&self) -> &'static str {
        use self::Tag::*;
        match *self {
            Byte(_) => stringify!(Byte),
            Short(_) => stringify!(Short),
            Int(_) => stringify!(Int),
            Long(_) => stringify!(Long),
            Float(_) => stringify!(Float),
            Double(_) => stringify!(Double),
            ByteArray(_) => stringify!(ByteArray),
            String(_) => stringify!(String),
            List(_) => stringify!(List),
            Compound(_) => stringify!(Compound),
            IntArray(_) => stringify!(IntArray),
        }
    }

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

    impl_as! {
        (as_byte;Byte;i8)
        (as_short;Short;i16)
        (as_int;Int;i32)
        (as_long;Long;i64)
        (as_float;Float;f32)
        (as_double;Double;f64)
    }

    impl_as_ref! {
        (as_byte_array;ByteArray;ByteArray)
        (as_string;String;str)
        (as_list;List;List)
        (as_compound;Compound;Compound)
        (as_int_array;IntArray;IntArray)
    }

    fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match *self {
            Tag::Byte(x) => binary::write_byte(w, x)?,
            Tag::Short(x) => binary::write_ishort(w, x)?,
            Tag::Int(x) => binary::write_int(w, x)?,
            Tag::Long(x) => binary::write_long(w, x)?,
            Tag::Float(x) => binary::write_float(w, x)?,
            Tag::Double(x) => binary::write_double(w, x)?,
            Tag::ByteArray(ref x) => x.write(w)?,
            Tag::String(ref x) => write_string(w, x)?,
            Tag::List(ref x) => x.write(w)?,
            Tag::Compound(ref x) => x.write(w)?,
            Tag::IntArray(ref x) => x.write(w)?,
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
            7 => Ok(Tag::ByteArray(ByteArray::read(r)?)),
            8 => Ok(Tag::String(read_string(r)?)),
            9 => Ok(Tag::List(List::read(r, level)?)),
            10 => Ok(Tag::Compound(Compound::read(r, level)?)),
            11 => Ok(Tag::IntArray(IntArray::read(r)?)),
            _ => Err(Error::InvalidID(id)),
        }
    }
}