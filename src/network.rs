use proto::{self, Reader, State, Writer};
use proto::packets::*;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use text;
use text::chat::{Chat, Component, StringComponent};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IOError(err: io::Error) {
            description(err.description())
            display("io error: {}", err)
            cause(err)
            from()
        }
    }
}

pub struct NetworkServer {
    favicon: Arc<Option<String>>,
    running: Arc<Mutex<bool>>,
    addr: SocketAddr,
}

impl NetworkServer {
    pub fn new(
        addr: SocketAddr,
        favicon: Option<String>,
        running: Arc<Mutex<bool>>,
    ) -> NetworkServer {
        NetworkServer {
            addr,
            favicon: Arc::new(favicon),
            running,
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.addr)?;

        let favicon = self.favicon.clone();
        let running = self.running.clone();

        thread::spawn(move || {
            loop {
                if !*running.lock().unwrap() {
                    break;
                }

                let favicon = favicon.clone();
                let (stream, addr) = listener.accept().unwrap();
                println!("{} has connected", addr);
                // TODO: async io
                thread::spawn(move || {
                    NetworkServer::handle_client(&stream, addr, favicon).unwrap_or_else(|e| {
                        println!("{} has been disconnected for error: {}", addr, e)
                    });
                });
            }
        });

        Ok(())
    }

    fn handle_client(
        stream: &TcpStream,
        addr: SocketAddr,
        favicon: Arc<Option<String>>,
    ) -> Result<(), proto::Error> {
        let mut reader = Reader::new(stream.try_clone().unwrap());
        let writer = Writer::new(stream.try_clone().unwrap());

        let packet = reader.read_packet()?;

        match packet {
            CPacket::Handshake { next_state, .. } => {
                if next_state == 1 {
                    NetworkServer::handle_status(reader, writer, favicon)
                } else {
                    NetworkServer::handle_login(reader, writer, addr)
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn handle_status<R: Read, W: Write>(
        mut reader: Reader<R>,
        mut writer: Writer<W>,
        favicon: Arc<Option<String>>,
    ) -> Result<(), proto::Error> {
        reader.set_state(State::Status);

        let mut packet = reader.read_packet()?;
        match packet {
            CPacket::StatusRequest {} => {}
            _ => {
                return Err(proto::Error::UnexpectedPacket {
                    expected: stringify!(CPacket::StatusRequest),
                    got: format!("{:?}", packet),
                });
            }
        }

        let response = SPacket::StatusResponse {
            data: SStatusResponseData {
                version: SStatusResponseVersion {
                    name: proto::VERSION_STRING,
                    protocol: proto::VERSION,
                },
                players: SStatusResponsePlayers {
                    max: 1000,
                    online: 1,
                    sample: None,
                },
                description: Chat::from(Component::from(StringComponent {
                    text: "A minecraft server".into(),
                    base: Default::default(),
                })),
                favicon,
            },
        };

        writer.write_packet(&response)?;

        packet = reader.read_packet()?;
        let payload = match packet {
            CPacket::StatusPing { payload } => payload,
            _ => {
                return Err(proto::Error::UnexpectedPacket {
                    expected: stringify!(CPacket::StatusPing),
                    got: format!("{:?}", packet),
                });
            }
        };

        writer.write_packet(&SPacket::StatusPong { payload })?;
        Ok(())
    }

    pub fn handle_login<R: Read, W: Write>(
        mut reader: Reader<R>,
        mut writer: Writer<W>,
        addr: SocketAddr,
    ) -> Result<(), proto::Error> {
        reader.set_state(State::Login);
        let packet = reader.read_packet()?;
        match packet {
            CPacket::LoginLoginStart { name } => {
                writer.write_packet(&SPacket::LoginDisconnect {
                    reason: Chat::from(Component::from(text::parse_legacy(
                        &format!("&4&lYou have been banned, {}", name),
                        '&',
                    ))),
                })?;
            }
            _ => {
                return Err(proto::Error::UnexpectedPacket {
                    expected: stringify!(CPacket::LoginLoginStart),
                    got: format!("{:?}", packet),
                });
            }
        }

        Ok(())
    }
}
