use binary;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;

mod error;
pub use self::error::*;

pub mod packets;
pub mod data;

pub const VERSION: i32 = 47;
pub const VERSION_STRING: &str = "1.8";
pub const MAX_PACKET_LEN: i32 = 2_097_152;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Handshake,
    Play,
    Status,
    Login
}

pub struct Writer<W: Write> {
    w: BufWriter<W>,
    buf: Vec<u8>,
}

impl<W: Write> Writer<W> {
    pub fn new(w: W) -> Writer<W> {
        Writer { w: BufWriter::new(w), buf: Vec::new() }
    }

    pub fn write_packet(&mut self, packet: &packets::SPacket) -> io::Result<()> {
        binary::write_varint(&mut self.buf, packet.id())?;
        packet.write(&mut self.buf)?;

        binary::write_varint(&mut self.w, self.buf.len() as i32)?;
        self.w.write_all(&self.buf)?;
        self.w.flush()?;

        self.buf.clear();
        Ok(())
    }
}

pub struct Reader<R: Read> {
    r: BufReader<R>,
    buf: Vec<u8>,
    state: State,
}

impl<R: Read> Reader<R> {
    pub fn new(r: R) -> Reader<R> {
        Reader { r: BufReader::new(r), buf: Vec::new(), state: State::Handshake }
    }

    pub fn set_state(&mut self, s: State) {
        self.state = s;
    }

    pub fn read_packet(&mut self) -> Result<packets::CPacket> {
        let len = binary::read_varint(&mut self.r)?;
        if len < 0 {
            return Err(Error::NegativePacketLen(len));
        } else if len > MAX_PACKET_LEN {
            return Err(Error::PacketSizeExceededMaxAllowed(len))
        }

        self.buf.resize(len as usize, 0);
        self.r.read_exact(&mut self.buf)?;

        let mut slice = &self.buf[..];
        let id = binary::read_varint(&mut slice)?;
        let packet = packets::CPacket::read(&mut slice, self.state, id)?;
        if !slice.is_empty() {
            return Err(Error::PacketTooLarge(slice.len()))
        } 

        Ok(packet)
    }
}