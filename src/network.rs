use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{
    collections::HashSet,
    thread::{self, JoinHandle},
};

use failure::Fail;
use uuid::{self, Uuid};

use crate::entity::player::Player;
use crate::proto::packets::SStatusResponsePlayer;
use crate::proto::packets::*;
use crate::proto::{self, Reader, State, Writer};
use crate::text;
use crate::text::chat::{Chat, Component, StringComponent};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "io error: {}", _0)]
    IOError(#[cause] io::Error),
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error::IOError(x)
    }
}

pub struct NetworkServer {
    favicon: Arc<Option<String>>,
    running: Arc<Mutex<bool>>,
    addr: SocketAddr,
    incoming_players: Sender<Player>,
    player_list: Arc<Mutex<HashSet<SStatusResponsePlayer>>>,
}

impl NetworkServer {
    pub fn new(
        addr: SocketAddr,
        favicon: Option<String>,
        running: Arc<Mutex<bool>>,
        incoming_players: Sender<Player>,
    ) -> NetworkServer {
        NetworkServer {
            addr,
            favicon: Arc::new(favicon),
            running,
            incoming_players,
            player_list: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.addr)?;

        let favicon = Arc::clone(&self.favicon);
        let running = Arc::clone(&self.running);
        let incoming_players = self.incoming_players.clone();
        let player_list = Arc::clone(&self.player_list);

        thread::spawn(move || {
            while *running.lock().unwrap() {
                let favicon = Arc::clone(&favicon);
                let incoming_players = incoming_players.clone();
                let player_list = Arc::clone(&player_list);

                let (stream, addr) = listener.accept().unwrap();
                println!("{} has connected", addr);
                // TODO: async io
                thread::spawn(move || {
                    if let Err(e) = NetworkServer::handle_client(
                        &stream,
                        addr,
                        favicon,
                        incoming_players,
                        player_list,
                    ) {
                        println!("{} has been disconnected for error: {:?}", addr, e);
                    }
                });
            }
        });

        Ok(())
    }

    fn handle_client(
        stream: &TcpStream,
        addr: SocketAddr,
        favicon: Arc<Option<String>>,
        incoming_players: Sender<Player>,
        player_list: Arc<Mutex<HashSet<SStatusResponsePlayer>>>,
    ) -> Result<(), proto::Error> {
        let mut reader = Reader::new(stream.try_clone().unwrap());
        let writer = Writer::new(stream.try_clone().unwrap());

        let packet = reader.read_packet()?;

        match packet {
            CPacket::Handshake { next_state, .. } => {
                if next_state == 1 {
                    NetworkServer::handle_status(reader, writer, favicon, player_list)
                } else {
                    NetworkServer::handle_login(reader, writer, addr, player_list, incoming_players)
                }
            }
            _ => unreachable!(), // reader is in handshake state so only handshake can be read.
        }
    }

    fn handle_status<R: Read + Send + 'static, W: Write + Send + 'static>(
        mut reader: Reader<R>,
        mut writer: Writer<W>,
        favicon: Arc<Option<String>>,
        player_list: Arc<Mutex<HashSet<SStatusResponsePlayer>>>,
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

        let online = player_list.lock().unwrap().len() as i32;
        let response = SPacket::StatusResponse {
            data: SStatusResponseData {
                version: SStatusResponseVersion {
                    name: proto::VERSION_STRING,
                    protocol: proto::VERSION,
                },
                players: SStatusResponsePlayers {
                    max: 1000,
                    online,
                    sample: Some(player_list),
                },
                description: Chat::from(Component::from(StringComponent {
                    text: "Quartz minecraft server".into(),
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

    fn handle_login<R: Read + Send + 'static, W: Write + Send + 'static>(
        mut reader: Reader<R>,
        mut writer: Writer<W>,
        addr: SocketAddr,
        player_list: Arc<Mutex<HashSet<SStatusResponsePlayer>>>,
        incoming_players: Sender<Player>,
    ) -> Result<(), proto::Error> {
        reader.set_state(State::Login);
        let packet = reader.read_packet()?;
        match packet {
            CPacket::LoginLoginStart { name } => {
                let uuid = Uuid::new_v3(&uuid::NAMESPACE_URL, &format!("OfflinePlayer:{}", name));

                let response_player = SStatusResponsePlayer {
                    name: name.clone(),
                    id: uuid.hyphenated().to_string(),
                };

                // check if player is already logged in
                if player_list.lock().unwrap().contains(&response_player) {
                    writer.write_packet(&SPacket::LoginDisconnect {
                        reason: Chat::from(text::parse_legacy("You are already logged in")),
                    })?;
                    return Ok(());
                }

                const THRESHOLD: u32 = 256;
                writer.write_packet(&SPacket::LoginSetCompression {
                    threshold: THRESHOLD as i32,
                })?;
                reader.set_compression(THRESHOLD);
                writer.set_compression(THRESHOLD);

                writer.write_packet(&SPacket::LoginLoginSuccess {
                    username: name.clone(),
                    uuid: uuid.hyphenated().to_string(),
                })?;

                let (server_sender, server_receiver) = mpsc::channel();
                let (client_sender, client_receiver) = mpsc::channel();

                let connected = Arc::new(Mutex::new(true));
                let mut player = Player::new(
                    name,
                    uuid,
                    server_sender,
                    client_receiver,
                    addr,
                    Arc::clone(&connected),
                );

                player_list.lock().unwrap().insert(response_player.clone());

                incoming_players.send(player).unwrap();

                NetworkServer::handle_play(
                    reader,
                    writer,
                    connected,
                    client_sender,
                    server_receiver,
                    response_player,
                    player_list,
                )
            }
            _ => {
                return Err(proto::Error::UnexpectedPacket {
                    expected: stringify!(CPacket::LoginLoginStart),
                    got: format!("{:?}", packet),
                });
            }
        }
    }

    // Repeatedly reads from client and sends to client.
    // When the server disconnects a player deliberately, connected_s will be set to false and other end of the channels will be dropped.
    // send_thread will return immediately then, and receiving thread will eventually return (when read_packet returns error or timeout)
    // TODO: make this return immediately as well
    // If an error occurs from this side from one thread, that thread will return,
    // connected_s will be set to false and the server will follow the same procedure as above.
    fn handle_play<R: Read + Send + 'static, W: Write + Send + 'static>(
        mut reader: Reader<R>,
        mut writer: Writer<W>,
        connected: Arc<Mutex<bool>>,
        sender: Sender<CPacket>,
        receiver: Receiver<Arc<SPacket>>,
        player: SStatusResponsePlayer,
        player_list: Arc<Mutex<HashSet<SStatusResponsePlayer>>>,
    ) -> Result<(), proto::Error> {
        reader.set_state(State::Play);
        let _player_guard = PlayerGuard {
            player_list,
            player,
        }; // remove player from player_list when returning from this function

        let connected_s = Arc::clone(&connected);
        // loop for packet sending.
        let send_thread: JoinHandle<Result<(), proto::Error>> = thread::spawn(move || {
            while *connected_s.lock().unwrap() {
                let packet = receiver.recv();
                match packet {
                    Ok(p) => {
                        if let Err(e) = writer.write_packet(&p) {
                            *connected_s.lock().unwrap() = false;
                            /*                            // normal disconnect
                            if e.kind() == io::ErrorKind::ConnectionAborted {
                                break;
                            }*/

                            return Err(From::from(e));
                        }
                    }
                    Err(_) => break, // send side is dropped when player is deallocated. That's gonna happen after player sets connected to false.
                }
            }

            Ok(())
        });

        let mut err = Ok(());
        // loop for packet receiving.
        while *connected.lock().unwrap() {
            let packet = reader.read_packet();
            match packet {
                Ok(p) => {
                    if let Err(_) = sender.send(p) {
                        break;
                    }
                }
                Err(e) => {
                    *connected.lock().unwrap() = false;
                    // normal disconnect
                    /*if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
                        if ioerr.kind() == io::ErrorKind::UnexpectedEof {
                            break;
                        }
                    }*/
                    err = Err(e);
                    break;
                }
            }
        }
        err.and(send_thread.join().unwrap())
    }
}

struct PlayerGuard {
    player_list: Arc<Mutex<HashSet<SStatusResponsePlayer>>>,
    player: SStatusResponsePlayer,
}

impl Drop for PlayerGuard {
    fn drop(&mut self) {
        self.player_list.lock().unwrap().remove(&self.player);
    }
}
