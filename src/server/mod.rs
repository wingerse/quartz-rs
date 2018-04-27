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
use std::collections::{HashMap, VecDeque, HashSet};
use std::cell::RefCell;

use base64;
use uuid::Uuid;

use network::{self, NetworkServer};
use entity::player::Player;
use proto::packets::{SPacket, SPlayPlayerListItemData, SPlayPlayerListItemDataAction};
use world::{Dimension, LevelType, World};
use world::chunk::{ChunkPos, Chunk};
use self::playerlist::PlayerList;
use text::{self, Code, ChatPos};
use text::chat::{Chat, Component};
use util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "io error: {}", _0)]
    IOError(#[cause] io::Error),
    #[fail(display = "network error: {}", _0)]
    NetworkError(#[cause] network::Error),
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self { Error::IOError(x) }
}

impl From<network::Error> for Error {
    fn from(x: network::Error) -> Self { Error::NetworkError(x) }
}

pub const MS_PER_TICK: u64 = 50;
pub const TICKS_PER_SEC: u64 = 1000 / MS_PER_TICK;

/// information players need to know about server
pub struct ServerInfo {
    pub view_distance: u8,
    pub player_view_distance: u8,
    pub difficulty: Difficulty,
    pub level_type: LevelType,
    pub tick: u64,
}

pub struct Worlds {
    overworld: World,
    nether: World,
    end: World,
}

impl Worlds {
    fn new() -> Worlds {
        Worlds {
            overworld: World::new(Dimension::Overworld),
            nether: World::new(Dimension::Nether),
            end: World::new(Dimension::End),
        }
    }

    fn get_world(&mut self, dimension: Dimension) -> &mut World {
        match dimension {
            Dimension::Overworld => &mut self.overworld,
            Dimension::Nether => &mut self.nether,
            Dimension::End => &mut self.end,
        }
    }
}

pub struct ServerContext<'a> {
    pub player_list: &'a PlayerList,
    pub world: &'a mut World,
    pub server_info: &'a ServerInfo,
}

pub struct Server {
    running: Arc<Mutex<bool>>,

    network_server: NetworkServer,
    player_list: PlayerList,
    incoming_players: Receiver<Player>,

    worlds: Worlds,

    current_time: Instant,
    start_time: Instant,
    next_entity_id: i32,
    server_info: ServerInfo,
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

            worlds: Worlds::new(),

            current_time: Instant::now(),
            start_time: Instant::now(),
            next_entity_id: 0,
            server_info: ServerInfo {
                view_distance: 10,
                player_view_distance: 3,
                difficulty: Difficulty::Peaceful,
                level_type: LevelType::Flat,
                tick: 0,
            },
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
            let mut millis = util::duration_total_ms(duration) as u64;

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

    fn get_next_entity_id(&mut self) -> i32 {
        let ret = self.next_entity_id;
        self.next_entity_id += 1;
        ret
    }

    fn add_new_player(&mut self, mut p: Player) {
        let entity_id = self.get_next_entity_id();
        p.set_entity_id(entity_id);
        p.set_join_tick(self.server_info.tick);
        let world = self.worlds.get_world(p.get_dimension());
        p.join(&mut ServerContext { server_info: &self.server_info, player_list: &self.player_list, world });
        self.player_list.add_player(p);
    }

    fn accept_new_players(&mut self) {
        loop {
            let p = self.incoming_players.try_recv();
            match p {
                Ok(player) => {
                    self.add_new_player(player);
                }
                Err(_) => break,
            }
        }
    }

    fn remove_disconnected_players(&mut self) {
        let mut to_remove = Vec::new();
        for p in self.player_list.iter() {
            let p = p.borrow();
            if !p.get_connected() {
                to_remove.push(p.get_uuid());
            }
        }

        for u in to_remove {
            let mut p = self.player_list.remove_by_uuid(&u).unwrap();
            let world = self.worlds.get_world(p.get_dimension());
            p.leave(&mut ServerContext { player_list: &self.player_list, server_info: &self.server_info, world });
        }
    }

    fn tick_players(&mut self) {
        for p in self.player_list.iter() {
            let mut p = p.borrow_mut();
            let world = self.worlds.get_world(p.get_dimension());
            p.tick(&mut ServerContext { player_list: &self.player_list, server_info: &self.server_info, world });
        }
    }

    fn tick(&mut self) {
        self.accept_new_players();
        self.tick_players();
        self.remove_disconnected_players();
        self.server_info.tick += 1;
    }

    fn cleanup(&mut self) {}
}