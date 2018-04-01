use std::io;
use std::error::Error as StdError;
use std::fmt::{self};
use std::string::FromUtf8Error;
use nbt;
use text::chat;
use binary;

pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IOError(err: io::Error) {
            description(err.description())
            display("io error: {}", err)
            from()
            cause(err)
        }
        PacketTooLong(err: usize) {
            description("packet too long")
            display(me) -> ("{}, got: {}", me.description(), err)
        }
        NBTError(err: nbt::Error) {
            description(err.description())
            display("nbt error: {}", err)
            from()
            cause(err)
        }
        InvalidUtf8String(err: FromUtf8Error) {
            description(err.description())
            display("invalid utf8 string error: {}", err)
            from()
            cause(err)
        }
        ChatError(err: chat::Error) {
            description(err.description())
            display("chat error: {}", err)
            from()
            cause(err)
        }
        VarintError(err: binary::VarintError) {
            description(err.description())
            display("varint error: {}", err)
            from()
            cause(err)
        }
    }
}