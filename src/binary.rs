use std::io::{self, Write, Read, BufRead};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use std::error::Error;
use std::fmt;

pub fn write_bool<W: Write>(w: &mut W, b: bool) -> io::Result<()> {
    let byte = if b { 1u8 } else { 0u8 };
    w.write_u8(byte)
}

pub fn read_bool<R: Read>(r: &mut R) -> io::Result<bool> {
    Ok(r.read_u8()? != 0)
}

pub fn write_ubyte<W: Write>(w: &mut W, b: u8) -> io::Result<()> {
    w.write_u8(b)
}

pub fn read_ubyte<R: Read>(r: &mut R) -> io::Result<u8> {
    r.read_u8()
}

pub fn write_byte<W: Write>(w: &mut W, b: i8) -> io::Result<()> {
    w.write_i8(b)
}

pub fn read_byte<R: Read>(r: &mut R) -> io::Result<i8> {
    r.read_i8()
}

pub fn write_ushort<W: Write>(w: &mut W, s: u16) -> io::Result<()> {
    w.write_u16::<BigEndian>(s)
}

pub fn read_ushort<R: Read>(r: &mut R) -> io::Result<u16> {
    r.read_u16::<BigEndian>()
}

pub fn write_ishort<W: Write>(w: &mut W, s: i16) -> io::Result<()> {
    w.write_i16::<BigEndian>(s)
}

pub fn read_ishort<R: Read>(r: &mut R) -> io::Result<i16> {
    r.read_i16::<BigEndian>()
}

pub fn write_int<W: Write>(w: &mut W, i: i32) -> io::Result<()> {
    w.write_i32::<BigEndian>(i)
}

pub fn read_int<R: Read>(r: &mut R) -> io::Result<i32> {
    r.read_i32::<BigEndian>()
}

pub fn write_long<W: Write>(w: &mut W, l: i64) -> io::Result<()> {
    w.write_i64::<BigEndian>(l)
}

pub fn read_long<R: Read>(r: &mut R) -> io::Result<i64> {
    r.read_i64::<BigEndian>()
}

pub fn write_float<W: Write>(w: &mut W, f: f32) -> io::Result<()> {
    w.write_f32::<BigEndian>(f)
}

pub fn read_float<R: Read>(r: &mut R) -> io::Result<f32> {
    r.read_f32::<BigEndian>()
}

pub fn write_double<W: Write>(w: &mut W, f: f64) -> io::Result<()> {
    w.write_f64::<BigEndian>(f)
}

pub fn read_double<R: Read>(r: &mut R) -> io::Result<f64> {
    r.read_f64::<BigEndian>()
}

#[derive(Debug)]
pub enum VarintError {
    TooLarge,
    IOErr(io::Error),
}

impl fmt::Display for VarintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VarintError::TooLarge => write!(f, "Varint or varlong is too large"),
            VarintError::IOErr(ref e) => e.fmt(f),
        }
    }
}

impl Error for VarintError {
    fn description(&self) -> &str {
        match *self {
            VarintError::TooLarge => "Varint Error",
            VarintError::IOErr(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            VarintError::TooLarge => None,
            VarintError::IOErr(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for VarintError {
    fn from(err: io::Error) -> Self {
        VarintError::IOErr(err)
    }
}

fn write_varint<W: Write>(w: &mut W, x: i32) -> io::Result<()> {
    let mut x = x as u32;
    let mut buf = [0u8; 5];

    let mut i = 0;
    while x >= 0x80 {
        buf[i] = x as u8 | 0x80;
        x >>= 7;
        i += 1;
    }
    buf[i] = x as u8;
    let n = i + 1;

    w.write_all(&buf[..n])
}

fn read_varint<R: BufRead>(r: &mut R) -> Result<i32, VarintError> {
    let mut x: u32 = 0;
    let mut i = 0;

    let mut buffer = [0u8; 1];
    loop {
        r.read_exact(&mut buffer)?;
        if i > 4 {
            return Err(VarintError::TooLarge);
        }
        let first7 = buffer[0] & 0x7f;
        x |= (first7 as u32) << (i * 7);
        // msb not set
        if buffer[0] & 0x80 != 0 {
            break;
        }
        i += 1;
    }

    Ok(x as i32)
}

fn write_varlong<W: Write>(w: &mut W, x: i64) -> io::Result<()> {
    let mut x = x as u64;
    let mut buf = [0u8; 10];

    let mut i = 0;
    while x >= 0x80 {
        buf[i] = x as u8 | 0x80;
        x >>= 7;
        i += 1;
    }
    buf[i] = x as u8;
    let n = i + 1;

    w.write_all(&buf[..n])
}

fn read_varlong<R: BufRead>(r: &mut R) -> Result<i64, VarintError> {
    let mut x: u64 = 0;
    let mut i = 0;

    let mut buffer = [0u8; 1];
    loop {
        r.read_exact(&mut buffer)?;
        if i > 9 {
            return Err(VarintError::TooLarge);
        }
        let first7 = buffer[0] & 0x7f;
        x |= (first7 as u64) << (i * 7);
        // msb not set
        if buffer[0] & 0x80 != 0 {
            break;
        }
        i += 1;
    }

    Ok(x as i64)
}