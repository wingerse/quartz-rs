mod error;
pub mod packets;
pub mod data;

pub use self::error::*;

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;

use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

use binary;
use self::packets::{SPacket, CPacket};

pub const VERSION: i32 = 47;
pub const VERSION_STRING: &str = "1.8";
pub const MAX_PACKET_LEN: i32 = 2_097_152;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Handshake,
    Play,
    Status,
    Login
}

pub struct Writer<W: Write> {
    w: BufWriter<W>,
    buf: Vec<u8>,
    compression_threshold: Option<u32>,
}

impl<W: Write> Writer<W> {
    pub fn new(w: W) -> Writer<W> {
        Writer { w: BufWriter::new(w), buf: Vec::new(), compression_threshold: None }
    }

    pub fn set_compression(&mut self, compression_threshold: u32) {
        self.compression_threshold = Some(compression_threshold);
    }

    pub fn disable_compression(&mut self) {
        self.compression_threshold = None;
    }

    pub fn write_packet(&mut self, packet: &SPacket) -> io::Result<()> {
        binary::write_varint(&mut self.buf, packet.id())?;
        packet.write(&mut self.buf)?;

        let uncompressed_len = self.buf.len();

        if let Some(threshold) = self.compression_threshold {
            if uncompressed_len >= threshold as usize {
                let mut encoder = ZlibEncoder::new(Vec::<u8>::new(), Compression::best());
                encoder.write_all(&self.buf[..])?;
                let compressed_data = encoder.finish()?;

                self.buf.clear();
                binary::write_varint(&mut self.buf, uncompressed_len as i32)?;
                binary::write_varint(&mut self.w, (self.buf.len() + compressed_data.len()) as i32)?;
                self.w.write_all(&self.buf[..])?;
                self.w.write_all(&compressed_data[..])?;
            } else {
                binary::write_varint(&mut self.w, (self.buf.len() + 1) as i32)?;
                binary::write_varint(&mut self.w, 0)?;
                self.w.write_all(&self.buf[..])?;
            }
        } else {
            binary::write_varint(&mut self.w, self.buf.len() as i32)?;
            self.w.write_all(&self.buf[..])?;
        }

        self.w.flush()?;
        self.buf.clear();
        Ok(())
    }
}

pub struct Reader<R: Read> {
    r: BufReader<R>,
    buf: Vec<u8>,
    state: State,
    compression_threshold: Option<u32>,
}

impl<R: Read> Reader<R> {
    pub fn new(r: R) -> Reader<R> {
        Reader { r: BufReader::new(r), buf: Vec::new(), state: State::Handshake, compression_threshold: None }
    }

    pub fn set_compression(&mut self, compression_threshold: u32) {
        self.compression_threshold = Some(compression_threshold);
    }

    pub fn disable_compression(&mut self) {
        self.compression_threshold = None;
    }

    pub fn set_state(&mut self, s: State) {
        self.state = s;
    }

    pub fn read_packet(&mut self) -> Result<CPacket> {
        let len = binary::read_varint(&mut self.r)?;
        if len <= 0 {
            return Err(Error::NegativePacketLen(len));
        } else if len > MAX_PACKET_LEN {
            return Err(Error::PacketSizeExceededMaxAllowed(len))
        }

        self.buf.resize(len as usize, 0);
        self.r.read_exact(&mut self.buf[..])?;

        let mut decompressed_data = Vec::<u8>::new();

        let mut slice = &self.buf[..];

        if let Some(threshold) = self.compression_threshold {
            let uncompressed_len = binary::read_varint(&mut slice)?;
            if uncompressed_len < 0 {
                return Err(Error::NegativeUncompressedLen(uncompressed_len));
            } else if uncompressed_len == 0 {
                // uncompressed. go down
            } else {
                if (uncompressed_len as u32) < threshold {
                    return Err(Error::CompressedBeforeThreshold);
                }
                {
                    let mut decoder = ZlibDecoder::new(&mut slice);
                    decoder.read_to_end(&mut decompressed_data)?;
                }
                slice = &decompressed_data[..];
            }
        }

        let id = binary::read_varint(&mut slice)?;
        let packet = CPacket::read(&mut slice, self.state, id)?;
        if !slice.is_empty() {
            return Err(Error::PacketTooLarge(slice.len()))
        } 

        Ok(packet)
    }
}