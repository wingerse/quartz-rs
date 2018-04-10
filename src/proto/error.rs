use binary;
use nbt;
use std::error::Error as StdError;
use std::io;
use std::string::FromUtf8Error;
use super::MAX_PACKET_LEN;
use text::chat;

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
        PacketTooLarge(err: usize) {
            description("packet too large")
            display(me) -> ("{}, got {} bytes left", me.description(), err)
        }
        PacketSizeExceededMaxAllowed(err: i32) {
            description("packet size exceeded maximum allowed")
            display(me) -> ("{}: {}, got {}", me.description(), MAX_PACKET_LEN, err)
        }
        UnexpectedPacket {expected: &'static str, got: String} {
            description("unexpected packet")
            display(me) -> ("{}: expected: {}, got: {}", me.description(), expected, got)
        }
        NegativePacketLen(err: i32) {
            description("negative packet length")
            display(me) -> ("{}, got {}", me.description(), err)
        }
        InvalidPacketId(err: i32) {
            description("invalid packet id")
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