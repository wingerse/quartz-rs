use crate::binary;
use crate::nbt;
use std::error::Error as StdError;
use std::io;
use std::string::FromUtf8Error;
use crate::proto::MAX_PACKET_LEN;
use crate::text::chat;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "io error: {}", _0)]
    IOError(#[cause] io::Error),
    #[fail(display = "packet too large, got {} bytes left", _0)]
    PacketTooLarge(usize),
    #[fail(display = "packet size exceeded maximum allowed:, 2097152, got {}", _0)]
    PacketSizeExceededMaxAllowed(i32),
    #[fail(display = "unexpected packet. expected: {}, got: {}", expected, got)]
    UnexpectedPacket {expected: &'static str, got: String},
    #[fail(display = "negative packet length, got {}", _0)]
    NegativePacketLen(i32),
    #[fail(display = "received packet was compressed before threshold size was reached")]
    CompressedBeforeThreshold,
    #[fail(display = "negative uncompressed length, got {}", _0)]
    NegativeUncompressedLen(i32),
    #[fail(display = "invalid packet id, got {}", _0)]
    InvalidPacketId(i32),
    #[fail(display = "nbt error: {}", _0)]
    NBTError(#[cause] nbt::Error),
    #[fail(display = "invalid utf8 string error: {}", _0)]
    InvalidUtf8String(#[cause] FromUtf8Error),
    #[fail(display = "chat error: {}", _0)]
    ChatError(#[cause] chat::Error),
    #[fail(display = "varint error: {}", _0)]
    VarintError(#[cause] binary::VarintError),
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self { Error::IOError(x) }
}

impl From<nbt::Error> for Error {
    fn from(x: nbt::Error) -> Self { Error::NBTError(x) }
}

impl From<FromUtf8Error> for Error {
    fn from(x: FromUtf8Error) -> Self { Error::InvalidUtf8String(x) }
}

impl From<chat::Error> for Error {
    fn from(x: chat::Error) -> Self { Error::ChatError(x) }
}

impl From<binary::VarintError> for Error {
    fn from(x: binary::VarintError) -> Self { Error::VarintError(x) }
}