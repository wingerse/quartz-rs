pub mod playerlist;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::{Instant, Duration};
use std::thread;
use std::collections::HashMap;
use std::cell::RefCell;

use base64;
use uuid::Uuid;

use network::{self, NetworkServer};
use entity::player::Player;
use proto::packets::SPacket;
use world::{Dimension, LevelType};
use self::playerlist::PlayerList;
use world::chunk_provider::ChunkProvider;
use world::chunk::ChunkPos;

pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IOError(err: io::Error) {
            description(err.description())
            display("io error: {}", err)
            from()
            cause(err)
        }
        NetworkError(err: network::Error) {
            description(err.description())
            display("network error: {}", err)
            from()
            cause(err)
        }
    }
}

pub const MS_PER_TICK: u64 = 50;

pub struct Server {
    running: Arc<Mutex<bool>>,
    network_server: NetworkServer,
    player_list: PlayerList,
    incoming_players: Receiver<Player>,
    current_time: Instant,
    start_time: Instant,
}

impl Server {
    fn load_favicon() -> Result<Option<String>, Error> {
        const FAVICON: &str = "favicon.png";
        if !Path::new(FAVICON).exists() {
            return Ok(None);
        }

        let mut f = File::open("favicon.png")?;
        let mut v = Vec::new();
        f.read_to_end(&mut v)?;

        Ok(Some(format!("data:image/png;base64,{}", base64::display::Base64Display::standard(&v))))
    }

    pub fn new(addr: SocketAddr) -> Result<Server, Error> {
        let running = Arc::new(Mutex::new(false));
        let favicon = Server::load_favicon()?;
        let (tx, rx) = mpsc::channel();
        let network_server = NetworkServer::new(
            addr,
            favicon,
            running.clone(),
            tx,
        );
        Ok(Server {
            running,
            network_server,
            player_list: PlayerList::new(),
            incoming_players: rx,
            current_time: Instant::now(),
            start_time: Instant::now(),
        })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        *self.running.lock().unwrap() = true;

        self.network_server.start()?;

        self.current_time = Instant::now();
        let mut accumulator = 0;

        while *self.running.lock().unwrap() {
            let current_time = Instant::now();
            let duration = current_time - self.current_time;
            let mut millis = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000_000;

            if millis > 2000 {
                println!("Ticks taking too long. Can't compensate too much. Skipping {} ticks", (millis - 2000) / MS_PER_TICK);
                millis = 2000;
            }

            accumulator += millis;
            self.current_time = current_time;

            while accumulator >= MS_PER_TICK {
                self.tick();
                accumulator -= MS_PER_TICK;
            }

            thread::sleep(Duration::from_millis(MS_PER_TICK - accumulator));
        }

        self.cleanup();
        Ok(())
    }

    fn add_new_player(&mut self, p: Player) {
        let player = Rc::new(RefCell::new(p));
        self.player_list.add_player(Rc::clone(&player));
        player.borrow_mut().send_packet(Arc::new(SPacket::PlayJoinGame {
            entity_id: 0,
            gamemode: Gamemode::Creative as u8,
            dimension: Dimension::Overworld as i8,
            difficulty: Difficulty::Peaceful as u8,
            max_players: 100,
            level_type: LevelType::Flat.as_str(),
            reduced_debug_info: false,
        }));
        player.borrow_mut().send_packet(Arc::new(SPacket::PlayPluginMessage {
            channel: "MC|Brand",
            data: "Quartz".as_bytes().to_vec(),
        }));
        player.borrow_mut().send_packet(Arc::new(SPacket::PlayServerDifficulty {
            difficulty: Difficulty::Peaceful as u8,
        }));
        player.borrow_mut().send_packet(Arc::new(SPacket::PlayPlayerAbilities {
            flags: 0x08 | 0x04,
            flying_speed: 0.05,
            field_of_view_modifier: 0.5,
        }));

        for z in -5..5 {
            for x in -5..5 {
                let chunk = ChunkProvider::load_chunk(ChunkPos::new(x, z));
                player.borrow_mut().send_packet(Arc::new(chunk.to_proto_chunk_data(true)));
            }
        }

        player.borrow_mut().send_packet(Arc::new(SPacket::PlaySpawnPosition {
            location: ::proto::data::Position {x: 0, y: 82, z: 0},
        }));
        player.borrow_mut().send_packet(Arc::new(SPacket::PlayPlayerPositionAndLook {
            x: 0.0,
            y: 82.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        }));
    }

    fn accept_new_players(&mut self) {
        loop {
            let p = self.incoming_players.try_recv();
            match p {
                Ok(player) => {
                    self.add_new_player(player);
                },
                Err(_) => break,
            }
        }
    }

    fn tick(&mut self) {
        self.accept_new_players();

    }

    fn cleanup(&mut self) {

    }
}